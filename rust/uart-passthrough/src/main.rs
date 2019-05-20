#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![no_std]
#![no_main]

use k210_hal::pac;
use k210_hal::prelude::*;
use k210_shared::board::def::io;
use k210_shared::soc::fpioa;
use k210_shared::soc::sysctl;
use nb::block;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let clocks = k210_hal::clock::Clocks::new();

    // Configure UARTHS (→host)
    let serial = p.UARTHS.constrain(115_200.bps(), &clocks);
    let (mut tx, mut rx) = serial.split();

    // Configure UART1 (→WIFI)
    sysctl::clock_enable(sysctl::clock::UART1);
    sysctl::reset(sysctl::reset::UART1);
    fpioa::set_function(io::WIFI_RX as u8, fpioa::function::UART1_TX);
    fpioa::set_function(io::WIFI_TX as u8, fpioa::function::UART1_RX);
    let wifi_serial = p.UART1.constrain(115_200.bps(), &clocks);
    let (mut wtx, mut wrx) = wifi_serial.split();

    // Relay characters between UARTs
    loop {
        if let Ok(ch) = wrx.read() {
            let _res = block!(tx.write(ch));
        }
        if let Ok(ch) = rx.read() {
            let _res = block!(wtx.write(ch));
        }
    }
}
