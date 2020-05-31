#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![no_std]
#![no_main]

use k210_hal::pac::Peripherals;
use k210_hal::prelude::*;
use k210_hal::serial::Serial;
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
    let p = Peripherals::take().unwrap();
    sysctl::pll_set_freq(sysctl::pll::PLL0, 800_000_000).unwrap();
    sysctl::pll_set_freq(sysctl::pll::PLL1, 300_000_000).unwrap();
    sysctl::pll_set_freq(sysctl::pll::PLL2, 45_158_400).unwrap();
    let clocks = k210_hal::clock::Clocks::new();

    // Configure UARTHS (→host)
    let mut serial = p.UARTHS.configure( DEFAULT_BAUD.bps(), &clocks);
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
    let mut wifi_serial = p.UART1.configure(DEFAULT_BAUD.bps(), &clocks);
    let (mut wtx, mut wrx) = wifi_serial.split();

    // Relay characters between UARTs
    loop {
        if !gpiohs::get_pin(0) { // IO16 == DTR pulled low for Out-of-Band command
            // OOB restores safe baudrate for UARTHS, to be sure we're able to recover from
            // sync failures
            let mut rate = DEFAULT_BAUD;
            let userial = Serial::join(tx, rx).free();
            serial = userial.configure(rate.bps(), &clocks);
            let s = serial.split();
            tx = s.0;
            rx = s.1;

            match block!(rx.read()).unwrap() {
                0x23 => { // Set baudrate
                    let mut d = [0u8; 4];
                    d.iter_mut().for_each(|x| { *x = block!(rx.read()).unwrap() });
                    rate = u32::from(d[0]) | (u32::from(d[1]) << 8) | (u32::from(d[2]) << 16) | (u32::from(d[3]) << 24);

                    // re-configure UARTHS at new rate
                    let userial = Serial::join(tx, rx).free();
                    serial = userial.configure(rate.bps(), &clocks);
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
            // re-configure UART1
            let wifi_userial = Serial::join(wtx, wrx).free();
            wifi_serial = wifi_userial.configure(rate.bps(), &clocks);
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
