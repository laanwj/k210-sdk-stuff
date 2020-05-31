#![no_std]
#![allow(dead_code)]
/** Buffered UART, using interrupts — currently only receiving is buffered because this is most
 * important, avoiding loss of data when the FIFO fills up. Buffered sending is slightly less
 * interesting without a fully fledged scheduling OS.
 */
// Yep, this is an awful hack, many things are hardcoded that should not be, just a proof of concept…
use bare_metal::Nr;
use core::sync::atomic::{AtomicUsize, Ordering};
use k210_hal::pac;
use k210_shared::soc::sysctl;
use pac::interrupt::Interrupt;
use riscv::asm;
use riscv::register::{mhartid, mie, mip, mstatus};

const UART_BUFSIZE: usize = 8192;
/** UART ring buffer */
struct UartInstance {
    buf: [u8; UART_BUFSIZE],
    /** writing happens at head */
    head: AtomicUsize,
    /** reading happens at tail, until tail==head */
    tail: AtomicUsize,
}

static mut UART1_INSTANCE_RECV: UartInstance = UartInstance {
    buf: [0; UART_BUFSIZE],
    head: AtomicUsize::new(0),
    tail: AtomicUsize::new(0),
};

/** UART IIR interrupt reason */
const UART_INTERRUPT_SEND: u8 = 0x02;
const UART_INTERRUPT_RECEIVE: u8 = 0x04;
const UART_INTERRUPT_CHARACTER_TIMEOUT: u8 = 0x0C;

/** Receive FIFO trigger */
const UART_RECEIVE_FIFO_1: u32 = 0;
const UART_RECEIVE_FIFO_4: u32 = 1;
const UART_RECEIVE_FIFO_8: u32 = 2;
const UART_RECEIVE_FIFO_14: u32 = 3;

/** Send FIFO trigger */
const UART_SEND_FIFO_0: u32 = 0;
const UART_SEND_FIFO_2: u32 = 1;
const UART_SEND_FIFO_4: u32 = 2;
const UART_SEND_FIFO_8: u32 = 3;

const UART_IER_ERBFI: u32 = 1;

/** Handle UARTx interrupt */
fn interrupt_uart1() {
    unsafe {
        let uart = pac::UART1::ptr();
        let irecv = &mut UART1_INSTANCE_RECV;

        match ((*uart).fcr_iir.read().bits() & 0xf) as u8 {
            UART_INTERRUPT_RECEIVE | UART_INTERRUPT_CHARACTER_TIMEOUT => {
                // Read recv FIFO into receive ringbuffer
                let mut head = irecv.head.load(Ordering::SeqCst);
                let tail = irecv.head.load(Ordering::SeqCst);
                while ((*uart).lsr.read().bits() & 1) != 0 {
                    irecv.buf[head] = ((*uart).rbr_dll_thr.read().bits() & 0xff) as u8;
                    head += 1;
                    if head == UART_BUFSIZE {
                        head = 0;
                    }
                    // TODO: signal overflows in a less catastropic way ?
                    if head == tail {
                        panic!("UART recv buffer overflow");
                    }
                }
                irecv.head.store(head, Ordering::SeqCst);
            }
            UART_INTERRUPT_SEND => {
                // TODO
            }
            _ => {}
        }
    }
}

/** PLIC interrupts */
#[allow(non_snake_case)]
#[no_mangle]
fn MachineExternal() {
    if mip::read().mext() {
        unsafe {
            let hartid = mhartid::read();
            let plic = pac::PLIC::ptr();
            let target = &(*plic).targets[hartid * 2];
            let int_num = target.claim.read().bits();
            let int = Interrupt::try_from(int_num as u8).unwrap();

            // Does this really need the 'disable other interrupts, change threshold' dance
            // as done in handle_irq_m_ext in plic.c?
            match int {
                Interrupt::UART1 => interrupt_uart1(),
                // We'll get a spurious UARTHS interrupt, ignore it
                Interrupt::UARTHS => {}
                _ => {
                    panic!(
                        "unknown machineexternal hart {}, int {:?}",
                        hartid, int
                    );
                }
            }

            // Perform IRQ complete
            target.claim.write(|w| w.bits(int_num));
        }
    }
}

/** Enable or disable a PLIC interrupt for the current core */
fn plic_irq_enable(interrupt: Interrupt, enabled: bool) {
    let targetid = mhartid::read() * 2;
    let irq_nr = interrupt.nr();
    unsafe {
        let plic = pac::PLIC::ptr();
        let bit = 1 << ((irq_nr as u32) % 32);
        if enabled {
            (*plic).target_enables[targetid].enable[(irq_nr as usize) / 32]
                .modify(|r, w| w.bits(r.bits() | bit));
        } else {
            (*plic).target_enables[targetid].enable[(irq_nr as usize) / 32]
                .modify(|r, w| w.bits(r.bits() & !bit));
        }
    }
}

/** Set interrupt priority (0-7) */
fn plic_set_priority(interrupt: Interrupt, priority: u32) {
    let irq_nr = interrupt.nr();
    unsafe {
        let plic = pac::PLIC::ptr();
        (*plic).priority[irq_nr as usize].write(|w| w.bits(priority));
    }
}

/** Initialize UART */
fn uart_init(baud_rate: u32) {
    let uart = pac::UART1::ptr();
    sysctl::clock_enable(sysctl::clock::UART1);
    sysctl::reset(sysctl::reset::UART1);

    // Hardcode these for now:
    let data_width = 8; // 8 data bits
    let stopbit_val = 0; // 1 stop bit
    let parity_val = 0; // No parity
    let divisor = sysctl::clock_get_freq(sysctl::clock::APB0) / baud_rate;
    let dlh = ((divisor >> 12) & 0xff) as u8;
    let dll = ((divisor >> 4) & 0xff) as u8;
    let dlf = (divisor & 0xf) as u8;
    unsafe {
        // Set Divisor Latch Access Bit (enables DLL DLH) to set baudrate
        (*uart).lcr.write(|w| w.bits(1 << 7));
        (*uart).dlh_ier.write(|w| w.bits(dlh.into()));
        (*uart).rbr_dll_thr.write(|w| w.bits(dll.into()));
        (*uart).dlf.write(|w| w.bits(dlf.into()));
        // Clear Divisor Latch Access Bit after setting baudrate
        (*uart)
            .lcr
            .write(|w| w.bits((data_width - 5) | (stopbit_val << 2) | (parity_val << 3)));
        // Write IER
        (*uart).dlh_ier.write(|w| w.bits(0x80)); /* THRE */
        // Write FCT
        (*uart)
            .fcr_iir
            .write(|w| w.bits(UART_RECEIVE_FIFO_4 << 6 | UART_SEND_FIFO_8 << 4 | 0x1 << 3 | 0x1));
    }
}

/** Enable or disable UART interrupt */
fn uart_enable_intr(recv: bool) {
    unsafe {
        let uart = pac::UART1::ptr();
        if recv {
            (*uart)
                .dlh_ier
                .modify(|r, w| w.bits(r.bits() | UART_IER_ERBFI));
            plic_set_priority(Interrupt::UART1, 6);
            plic_irq_enable(Interrupt::UART1, true);
        } else {
            (*uart)
                .dlh_ier
                .modify(|r, w| w.bits(r.bits() & !UART_IER_ERBFI));
            plic_set_priority(Interrupt::UART1, 0);
            plic_irq_enable(Interrupt::UART1, false);
        }
    }
}

/** Send data to UART (blocking) */
pub fn send(s: &[u8]) {
    let uart = pac::UART1::ptr();
    for &c in s {
        unsafe {
            while ((*uart).lsr.read().bits() & (1 << 5)) != 0 {}
            (*uart).rbr_dll_thr.write(|w| w.bits(c.into()));
        }
    }
}

/** Receive data from UART (non-blocking, returns number of bytes received) */
pub fn recv_nb(s: &mut [u8]) -> usize {
    let irecv = unsafe { &mut UART1_INSTANCE_RECV };
    let head = irecv.head.load(Ordering::SeqCst);
    let mut tail = irecv.tail.load(Ordering::SeqCst);
    if head == tail { // Early-out without tail.store if ring buffer empty
        return 0;
    }
    let mut ptr = 0;
    while ptr < s.len() && tail != head {
        s[ptr] = irecv.buf[tail];
        tail += 1;
        if tail == UART_BUFSIZE {
            tail = 0;
        }
        ptr += 1;
    }
    irecv.tail.store(tail, Ordering::SeqCst);
    ptr
}

/** Receive data from UART (blocks for at least one byte if the buffer can hold one, returns number
 * of bytes received) */
pub fn recv(s: &mut [u8]) -> usize {
    if s.len() == 0 {
        return 0;
    }
    loop {
        let n = recv_nb(s);
        if n != 0 {
            return n;
        }
        unsafe { asm::wfi(); }
    }
}

/** Initialize interrupts and buffered UART handling */
pub fn init() {
    unsafe {
        // Enable interrupts in general
        mstatus::set_mie();
        // Set the Machine-Software bit in MIE
        mie::set_msoft();
        // Set the Machine-External bit in MIE
        mie::set_mext();
    }

    uart_init(115_200);
    uart_enable_intr(true);
}
