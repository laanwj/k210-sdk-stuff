#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![no_std]
#![no_main]

use k210_hal::Peripherals;
use k210_hal::pac;
use k210_hal::prelude::*;
use k210_hal::stdout::Stdout;
use riscv_rt::entry;
use k210_shared::soc::sleep::usleep;
use core::ptr;
use riscv::register::{mie,mstatus,mhartid,mvendorid,marchid,mimpid,mcause};
use core::sync::atomic::{AtomicBool,Ordering};

fn peek<T>(addr: u64) -> T {
    unsafe { ptr::read_volatile(addr as *const T) }
}

static INTR: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Copy, Clone)]
struct IntrInfo {
    hartid: usize,
    cause: mcause::Trap,
}

static mut INTR_INFO: Option<IntrInfo> = None;

#[no_mangle]
fn my_trap_handler() {
    let hartid = mhartid::read();
    let cause = mcause::read().cause();

    unsafe { INTR_INFO = Some(IntrInfo { hartid, cause }); }

    INTR.store(true, Ordering::SeqCst);
    unsafe {
        (*pac::CLINT::ptr()).msip[hartid].write(|w| w.bits(0));
    }
}

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let clocks = k210_hal::clock::Clocks::new();

    usleep(200000);

    // Configure UART
    let serial = p.UARTHS.configure((p.pins.pin5, p.pins.pin4), 115_200.bps(), &clocks);
    let (mut tx, _) = serial.split();

    let mut stdout = Stdout(&mut tx);

    //let x: u32 = peek::<u32>(0x80000000);
    //writeln!(stdout, "the value is {:08x}", x).unwrap();
    writeln!(stdout, "Some CPU information !").unwrap();
    writeln!(stdout, "  mvendorid {:?}", mvendorid::read()).unwrap();
    writeln!(stdout, "  marchid {:?}", marchid::read()).unwrap();
    writeln!(stdout, "  mimpid {:?}", mimpid::read()).unwrap();
    writeln!(stdout, "This code is running on hart {}", mhartid::read()).unwrap();

    writeln!(stdout, "Enabling interrupts").unwrap();

    unsafe {
        // Enable interrupts in general
        mstatus::set_mie();
        // Set the Machine-Software bit in MIE
        mie::set_msoft();
        // Set the Machine-External bit in MIE
        //mie::set_mext();
    }

    writeln!(stdout, "Generate IPI for core 0 !").unwrap();
    unsafe {
        (*pac::CLINT::ptr()).msip[0].write(|w| w.bits(1));
    }

    writeln!(stdout, "Waiting for interrupt").unwrap();
    while !INTR.load(Ordering::SeqCst) {
    }
    INTR.store(false, Ordering::SeqCst);
    writeln!(stdout, "Interrupt was triggered {:?}", unsafe { INTR_INFO }).unwrap();

    /*
    writeln!(stdout, "Generate IPI for core 1 !").unwrap();
    unsafe {
        (*pac::CLINT::ptr()).msip[1].write(|w| w.bits(1));
    }
    writeln!(stdout, "Waiting for interrupt").unwrap();
    while !INTR.load(Ordering::SeqCst) {
    }
    INTR.store(false, Ordering::SeqCst);
    writeln!(stdout, "Interrupt was triggered {:?}", unsafe { INTR_INFO }).unwrap();
    */
    
    writeln!(stdout, "[end]").unwrap();
    loop {
    }
}
