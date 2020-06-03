/*! MMU and interrupt experiments. */
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_macros)]
#![no_std]
#![no_main]
#![feature(llvm_asm)]

use k210_hal::pac;
use k210_hal::prelude::*;
use riscv_rt::{entry,TrapFrame};
use k210_shared::soc::sleep::usleep;
use k210_shared::soc::sysctl;
use k210_shared::{debugln,debug};
use core::ptr;
use riscv::register::{mie,mstatus,mhartid,mvendorid,marchid,mimpid,mcause};
use core::sync::atomic::{AtomicBool,Ordering};

#[macro_use]
mod aligned_as;

/* Pages are 4kiB on RISC-V */
const PAGESZ: usize = 4096;
const PAGEMASK: usize = PAGESZ - 1;
const PAGESHIFT: usize = 12;

/* Page table entry (PTE) fields */
const PTE_V: usize = 0x001; /* Valid */
const PTE_R: usize = 0x002; /* Read */
const PTE_W: usize = 0x004; /* Write */
const PTE_X: usize = 0x008; /* Execute */
const PTE_U: usize = 0x010; /* User */
const PTE_G: usize = 0x020; /* Global */
const PTE_A: usize = 0x040; /* Accessed */
const PTE_D: usize = 0x080; /* Dirty */
const PTE_SOFT: usize = 0x300; /* Reserved for Software */

const PTE_PPN_SHIFT: usize = 10;

/* Relevant CSRs */
const CSR_SPTBR: u16 = 0x180;
const CSR_MSTATUS: u16 = 0x300;
const CSR_MEPC: u16 = 0x341;
const CSR_MBADADDR: u16 = 0x343; // =mtval

/* mstatus CSR bitfields */
const MSTATUS_MPRV: usize = 0x00020000;
const MSTATUS_MPP: usize = 0x00001800;
const MSTATUS_MPP_SHIFT: usize = 11;
const MSTATUS_VM: usize = 0x1F000000;
const MSTATUS_VM_SV39: usize = 9 << 24;

/* Privilege levels */
const PRV_U: usize = 0;
const PRV_S: usize = 1;
const PRV_H: usize = 2;
const PRV_M: usize = 3;

/** Macro to read a CSR. */
macro_rules! csrr {
    ($csr_number:expr) => {
        {
            let r: usize;
            llvm_asm!("csrrs $0, $1, x0" : "=r"(r) : "i"($csr_number) :: "volatile");
            r
        }
    }
}

/** Macro to write an arbitrary CSR. */
macro_rules! csrw {
    ($csr_number:expr, $value:expr) => {
        {
            let w = $value;
            llvm_asm!("csrrw x0, $1, $0" :: "r"(w), "i"($csr_number) :: "volatile");
        }
    }
}

/** Macro to atomically set bits in an arbitrary CSR. */
macro_rules! csrs {
    ($csr_number:expr, $value:expr) => {
        {
            let w = $value;
            llvm_asm!("csrrs x0, $1, $0" :: "r"(w), "i"($csr_number) :: "volatile");
        }
    }
}

/** Macro to atomically clear bits in an arbitrary CSR. */
macro_rules! csrc {
    ($csr_number:expr, $value:expr) => {
        {
            let w = $value;
            llvm_asm!("csrrc x0, $1, $0" :: "r"(w), "i"($csr_number) :: "volatile");
        }
    }
}

/** Supervisor VM fence. */
fn sfence_vm() {
    unsafe {
        // Use raw bit pattern because LLVM assembler doesn't recognize the instruction.
        llvm_asm!(".word 0x10400073" :::: "volatile");
    }
}

/** Supervisor VM fence (with virtual address). */
fn sfence_vm_addr(addr: usize) {
    unsafe {
        // Clobber a fixed register because it's not possible otherwise with a
        // hard-coded bit pattern.
        llvm_asm!("
            mv t0, $0
            .word 0x10428073
        " :: "r"(addr) : "t0" : "volatile");
    }
}

/** One page of the page table, containing Page Table Entries (PTEs).
 * Always aligned to a page.
 */
#[derive(Copy, Clone)]
#[repr(C, align(4096))]
struct PageTable {
    entries: [usize; PAGESZ / 8],
}

impl PageTable {
    const N: usize = PAGESZ / 8;

    const fn new() -> Self {
        Self {
            entries: [0; Self::N],
        }
    }
}

/** Read an arbitrary memory address (for testing only). */
unsafe fn peek<T>(addr: u64) -> T {
    ptr::read_volatile(addr as *const T)
}

/** Write an arbitrary memory address (for testing only). */
unsafe fn poke<T>(addr: u64, value: T) {
    ptr::write_volatile(addr as *mut T, value)
}

/** Page tables. */
static mut PT: [&'static mut [PageTable]; 3] = [
    /* Page table level 0 (root) */
    &mut [PageTable::new(); 1],
    /* Page table level 1 */
    &mut [PageTable::new(); 1],
    /* Page table level 2 (leaf) */
    &mut [PageTable::new(); 16],
];

/** Read a PTE. */
unsafe fn pte_get(lvl: usize, idx: usize) -> (usize, usize) {
    let val = PT[lvl][idx / PageTable::N].entries[idx % PageTable::N];
    (((val >> PTE_PPN_SHIFT) << PAGESHIFT), val & ((1 << PTE_PPN_SHIFT) - 1))
}

/** Write a PTE.
 * To clear an entry, pass address and flags as 0.
 */
unsafe fn pte_put(lvl: usize, idx: usize, addr: usize, flags: usize) {
    assert!((addr & PAGEMASK) == 0);
    PT[lvl][idx / PageTable::N].entries[idx % PageTable::N] = ((addr >> PAGESHIFT) << PTE_PPN_SHIFT) | flags;
}

/** Reset the page tables to their initial state.
 * It sets up the first 4GB is simply a mirror of physical memory (as software expects).
 * After that, add umapped pages of virtual memory (how much depends on the hardcoded
 * sizes of the page tables).
 */
unsafe fn pte_reset() -> usize {
    let lvl0_base = PT[0].as_ptr() as usize;
    let lvl1_base = PT[1].as_ptr() as usize;
    let lvl2_base = PT[2].as_ptr() as usize;

    // The first 4GB is simply a mirror of physical memory
    for i in 0..4 {
        pte_put(0, i, 0x40000000 * i,  PTE_V | PTE_R | PTE_W | PTE_X | PTE_G | PTE_U);
    }

    // Then follows our custom page tables as a linear span of virtual memory
    // Level 0 points to subsequent pages of level 1.
    for i in 0..PT[1].len() {
        pte_put(0, 4 + i, lvl1_base + i * PAGESZ, PTE_V | PTE_G | PTE_U);
    }

    // Level 1 points to subsequent pages of level 2.
    for i in 0..PT[2].len() {
        pte_put(1, i, lvl2_base + i * PAGESZ, PTE_V | PTE_G | PTE_U);
    }

    // Start level 2 as all-invalid.
    for page in PT[2].iter_mut() {
        *page = PageTable::new();
    }

    return lvl0_base;
}

#[derive(Debug, Copy, Clone)]
struct IntrInfo {
    hartid: usize,
    cause: mcause::Trap,
}

static INTR: AtomicBool = AtomicBool::new(false);
static CORE1ON: AtomicBool = AtomicBool::new(false);
static mut INTR_INFO: Option<IntrInfo> = None;
static TEST_PAGE: &'static [u8] = include_bytes_align_as!(PageTable, "testpage.dat");

/** Send interprocessor interrupt. */
fn send_ipi(hart: usize) {
    unsafe {
        (*pac::CLINT::ptr()).msip[hart].write(|w| w.bits(1));
    }
}

/** Clear interprocessor interrupt. */
fn clear_ipi(hart: usize) {
    unsafe {
        (*pac::CLINT::ptr()).msip[hart].write(|w| w.bits(0));
    }
}

/** Handle external interrupts. */
#[allow(non_snake_case)]
#[no_mangle]
fn MachineSoft() {
    let hartid = mhartid::read();
    let cause = mcause::read().cause();

    unsafe { INTR_INFO = Some(IntrInfo { hartid, cause }); }

    INTR.store(true, Ordering::SeqCst);
    clear_ipi(hartid);
}

/** Handle CPU exceptions. */
#[allow(non_snake_case)]
#[no_mangle]
pub fn ExceptionHandler(_trap_frame: &TrapFrame) {
    let cause = mcause::read().cause();
    let badaddr = unsafe { csrr!(CSR_MBADADDR) };
    match cause {
        mcause::Trap::Exception(mcause::Exception::LoadFault) => {
            // Map the test page address when requested
            if badaddr == 0x1_0123_0000 {
                mmu_map(0x1_0123_0000, TEST_PAGE.as_ptr() as usize, 1, PTE_R | PTE_W | PTE_X);
                return;
            }
        }
        _ => {}
    }
    debugln!("exception handler called (cause {:?}, badaddr {:08x})", cause, badaddr);
    loop {
        continue;
    }
}

/** Set up page tables and MMU. */
fn setup_mmu() {
    assert_eq!(core::mem::size_of::<PageTable>(), PAGESZ);
    assert_eq!(core::mem::align_of::<PageTable>(), PAGESZ);

    unsafe {
        let lvl0_base = PT[0].as_ptr() as usize;
        let lvl1_base = PT[1].as_ptr() as usize;
        let lvl2_base = PT[2].as_ptr() as usize;
        debugln!("pagetables lvl0: {:08x} lvl1: {:08x} lvl2: {:08x}",
                 lvl0_base, lvl1_base, lvl2_base);

        let pt_root = pte_reset();

        // Set Supervisor Page-Table Base Register
        csrw!(CSR_SPTBR, pt_root >> PAGESHIFT);

        let mut mstatus = csrr!(CSR_MSTATUS);

        // Enable SV39 virtual memory system
        mstatus &= !MSTATUS_VM;
        mstatus |= MSTATUS_VM_SV39;

        // Do loads and stores with User privilege
        mstatus &= !MSTATUS_MPP;
        mstatus |= MSTATUS_MPRV | (PRV_U << MSTATUS_MPP_SHIFT);

        csrw!(CSR_MSTATUS, mstatus);

        sfence_vm();

        debugln!("MMU enabled");
    }
}

/** Map pages from physical to virtual memory.
 * flags is a combination of PTE_R, PTR_W, PTR_X.
 */
fn mmu_map(mut vaddr: usize, mut paddr: usize, npages: usize, flags: usize) {
    for _ in 0..npages {
        let idx = (vaddr - 0x1_0000_0000) >> PAGESHIFT;
        unsafe {
            pte_put(2, idx, paddr, PTE_V | PTE_G | PTE_U | flags);
        }
        sfence_vm_addr(vaddr);
        vaddr += PAGESZ;
        paddr += PAGESZ;
    }
}

/** Unmap pages from virtual memory.
 */
fn mmu_unmap(mut vaddr: usize, npages: usize) {
    for _ in 0..npages {
        let idx = (vaddr - 0x1_0000_0000) >> PAGESHIFT;
        unsafe {
            pte_put(2, idx, 0, 0);
        }
        sfence_vm_addr(vaddr);
        vaddr += PAGESZ;
    }
}

#[export_name = "_mp_hook"]
pub extern "Rust" fn mp_hook() -> bool {
    return mhartid::read() == 0;
}

#[entry]
fn main() -> ! {
    if mhartid::read() == 1 {
        unsafe {
            // Clear pending IPI that activated core 1
            clear_ipi(1);
            // Enable interrupts in general
            mstatus::set_mie();
            // Set the Machine-Software bit in MIE
            mie::set_msoft();
        }
        CORE1ON.store(true, Ordering::SeqCst);
        loop {
            unsafe { riscv::asm::wfi() }
        }
    }

    let p = pac::Peripherals::take().unwrap();
    sysctl::pll_set_freq(sysctl::pll::PLL0, 800_000_000).unwrap();
    sysctl::pll_set_freq(sysctl::pll::PLL1, 300_000_000).unwrap();
    sysctl::pll_set_freq(sysctl::pll::PLL2, 45_158_400).unwrap();
    let clocks = k210_hal::clock::Clocks::new();

    usleep(200000);

    p.UARTHS.configure(115_200.bps(), &clocks);

    //let x: u32 = peek::<u32>(0x80000000);
    debugln!("Some CPU information !");
    debugln!("  mvendorid {:?}", mvendorid::read());
    debugln!("  marchid {:?}", marchid::read());
    debugln!("  mimpid {:?}", mimpid::read());
    debugln!("This code is running on hart {}", mhartid::read());

    debugln!("Enabling interrupts");

    unsafe {
        // Enable interrupts in general
        mstatus::set_mie();
        // Set the Machine-Software bit in MIE
        mie::set_msoft();
        // Set the Machine-External bit in MIE
        //mie::set_mext();
    }

    debugln!("Generate IPI for core 0 !");
    send_ipi(0);

    debugln!("Waiting for interrupt");
    while !INTR.load(Ordering::SeqCst) {
    }
    INTR.store(false, Ordering::SeqCst);
    debugln!("Interrupt was triggered {:?}", unsafe { INTR_INFO });

    setup_mmu();

    /* Print some stuff from a demand-mapped page. */
    for x in 0..10 {
        let val = unsafe { peek::<u8>(0x1_0123_0000 + x) };
        debug!("{}", val as char);
    }

    /* Second core testing. */
    debugln!("Generate IPI for core 1 !");
    send_ipi(1);
    debugln!("Waiting for core 1 to come up");
    while !CORE1ON.load(Ordering::SeqCst) {
    }
    debugln!("Core 1 reported active");

    debugln!("Generate IPI for core 1 again");
    send_ipi(1);
    debugln!("Waiting for interrupt");
    while !INTR.load(Ordering::SeqCst) {
    }
    INTR.store(false, Ordering::SeqCst);
    debugln!("Interrupt was triggered {:?}", unsafe { INTR_INFO });
    
    debugln!("[end]");
    loop {
        continue;
    }
}
