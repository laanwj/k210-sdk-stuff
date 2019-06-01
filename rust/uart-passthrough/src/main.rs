#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![no_std]
#![no_main]

use k210_hal::pac;
use k210_hal::prelude::*;
use k210_shared::board::def::io;
use k210_shared::soc::fpioa;
use k210_shared::soc::gpio;
use k210_shared::soc::gpiohs;
use k210_shared::soc::sleep::usleep;
use k210_shared::soc::sysctl;
use nb::block;
use riscv_rt::entry;

const DEFAULT_BAUD: u32 = 115_200;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let clocks = k210_hal::clock::Clocks::new();

    // Configure UARTHS (→host)
    let mut serial = p.UARTHS.constrain(DEFAULT_BAUD.bps(), &clocks);
    let (mut tx, mut rx) = serial.split();

    // Configure UART1 (→WIFI)
    sysctl::clock_enable(sysctl::clock::UART1);
    sysctl::reset(sysctl::reset::UART1);
    fpioa::set_function(io::WIFI_RX, fpioa::function::UART1_TX);
    fpioa::set_function(io::WIFI_TX, fpioa::function::UART1_RX);
    fpioa::set_function(io::WIFI_EN, fpioa::function::GPIOHS8);
    fpioa::set_io_pull(io::WIFI_EN, fpioa::pull::DOWN);
    gpiohs::set_pin(8, true);
    gpiohs::set_direction(8, gpio::direction::OUTPUT);
    let mut wifi_serial = p.UART1.constrain(DEFAULT_BAUD.bps(), &clocks);
    let (mut wtx, mut wrx) = wifi_serial.split();

    // Relay characters between UARTs
    loop {
        if !gpiohs::get_pin(0) { // IO16 == DTR pulled low for Out-of-Band command
            // OOB restores safe baudrate for UARTHS, to be sure we're able to recover from
            // sync failures
            let mut rate = DEFAULT_BAUD;
            serial = rx.join(tx).free().constrain(rate.bps(), &clocks);
            let s = serial.split();
            tx = s.0;
            rx = s.1;

            match block!(rx.read()).unwrap() {
                0x23 => { // Set baudrate
                    let mut d = [0u8; 4];
                    d.iter_mut().for_each(|x| { *x = block!(rx.read()).unwrap() });
                    rate = (d[0] as u32) | ((d[1] as u32) << 8) | ((d[2] as u32) << 16) | ((d[3] as u32) << 24);

                    // re-constrain UARTHS at new rate
                    serial = rx.join(tx).free().constrain(rate.bps(), &clocks);
                    let s = serial.split();
                    tx = s.0;
                    rx = s.1;
                }
                0x42 => { // WIFI reset
                    gpiohs::set_pin(8, false);
                    usleep(100000);
                    gpiohs::set_pin(8, true);
                }
                _ => {}
            }
            // re-constrain UART1
            wifi_serial = wrx.join(wtx).free().constrain(rate.bps(), &clocks);
            let s = wifi_serial.split();
            wtx = s.0;
            wrx = s.1;
        }
        if let Ok(ch) = wrx.read() {
            let _res = block!(tx.write(ch));
        }
        if let Ok(ch) = rx.read() {
            let _res = block!(wtx.write(ch));
        }
    }
}
