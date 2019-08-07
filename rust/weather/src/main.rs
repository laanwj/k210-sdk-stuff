#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![no_std]
#![no_main]

use core::str;
use embedded_hal::serial;
use esp8266at::handler::{NetworkEvent, SerialNetworkHandler};
use esp8266at::response::{parse, ConnectionType, ParseResult};
use esp8266at::traits::{self, Write};
use k210_hal::Peripherals;
use k210_hal::prelude::*;
use k210_hal::stdout::Stdout;
use k210_shared::board::def::io;
use k210_shared::board::lcd::{self, LCD, LCDHL};
use k210_shared::soc::dmac::{DMACExt, dma_channel};
use k210_shared::soc::fpioa;
use k210_shared::soc::gpio;
use k210_shared::soc::gpiohs;
use k210_shared::soc::sleep::usleep;
use k210_shared::soc::spi::SPIExt;
use k210_shared::soc::sysctl;
use nb::block;
use riscv::register::mcycle;
use riscv_rt::entry;
use k210_console::console::{Console, ScreenImage, DISP_HEIGHT, DISP_WIDTH};

mod config;

const DEFAULT_BAUD: u32 = 115_200;
const TIMEOUT: usize = 390_000_000 * 40 / 115200;

struct WriteAdapter<'a, TX>
where
    TX: serial::Write<u8>,
{
    tx: &'a mut TX,
}
impl<'a, TX> WriteAdapter<'a, TX>
where
    TX: serial::Write<u8>,
    TX::Error: core::fmt::Debug,
{
    fn new(tx: &'a mut TX) -> Self {
        Self { tx }
    }
}
impl<'a, TX> traits::Write for WriteAdapter<'a, TX>
where
    TX: serial::Write<u8>,
    TX::Error: core::fmt::Debug,
{
    type Error = TX::Error;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        for ch in buf {
            block!(self.tx.write(*ch))?;
        }
        Ok(())
    }
}

/** Connect pins to internal functions */
fn io_init() {
    /* Init SPI IO map and function settings */
    fpioa::set_function(
        io::LCD_RST,
        fpioa::function::gpiohs(lcd::RST_GPIONUM),
    );
    fpioa::set_io_pull(io::LCD_RST, fpioa::pull::DOWN); // outputs must be pull-down
    fpioa::set_function(io::LCD_DC, fpioa::function::gpiohs(lcd::DCX_GPIONUM));
    fpioa::set_io_pull(io::LCD_DC, fpioa::pull::DOWN);
    fpioa::set_function(io::LCD_CS, fpioa::function::SPI0_SS3);
    fpioa::set_function(io::LCD_WR, fpioa::function::SPI0_SCLK);

    sysctl::set_spi0_dvp_data(true);

    /* Set dvp and spi pin to 1.8V */
    sysctl::set_power_mode(sysctl::power_bank::BANK6, sysctl::io_power_mode::V18);
    sysctl::set_power_mode(sysctl::power_bank::BANK7, sysctl::io_power_mode::V18);
}

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let clocks = k210_hal::clock::Clocks::new();

    usleep(200000);
    io_init();

    // Configure UARTHS (→host)
    let serial = p.UARTHS.configure((p.pins.pin5, p.pins.pin4), DEFAULT_BAUD.bps(), &clocks);
    let (mut tx, mut _rx) = serial.split();
    let mut debug = Stdout(&mut tx);

    // Configure UART1 (→WIFI)
    sysctl::clock_enable(sysctl::clock::UART1);
    sysctl::reset(sysctl::reset::UART1);
    fpioa::set_function(io::WIFI_EN, fpioa::function::GPIOHS8);
    fpioa::set_io_pull(io::WIFI_EN, fpioa::pull::DOWN);
    gpiohs::set_pin(8, true);
    gpiohs::set_direction(8, gpio::direction::OUTPUT);
    let wifi_serial = p.UART1.configure((p.pins.pin7, p.pins.pin6), DEFAULT_BAUD.bps(), &clocks);
    let (mut wtx, mut wrx) = wifi_serial.split();

    let mut wa = WriteAdapter::new(&mut wtx);
    let mut sh = SerialNetworkHandler::new(&mut wa, config::APNAME.as_bytes(), config::APPASS.as_bytes());

    // LCD ini
    let dmac = p.DMAC.configure();
    let spi = p.SPI0.constrain();
    let mut lcd = LCD::new(spi, &dmac, dma_channel::CHANNEL0);
    lcd.init();
    lcd.set_direction(lcd::direction::YX_LRUD);
    let mut console: Console = Console::new();

    writeln!(console, "\x1b[48;2;128;192;255;38;5;0m WEATHER \x1b[0m \x1b[38;2;128;128;128m\x1b[0m").unwrap();

    // Start off connection process state machine
    sh.start(false).unwrap();
    writeln!(console, "∙ Connecting to AP").unwrap();

    let mut serial_buf = [0u8; 8192];
    let mut ofs: usize = 0;

    let mut cur_link = 0;
    let mut finished = false;
    loop {
        if console.dirty {
            let mut image: ScreenImage = [0; DISP_WIDTH * DISP_HEIGHT / 2];
            console.render(&mut image);
            lcd.draw_picture(0, 0, DISP_WIDTH as u16, DISP_HEIGHT as u16, &image);
            console.dirty = false;
        }

        // When finished, wait around a bit and re-do request
        // do this after updating the console, to be sure the last result is visible before
        // sleeping
        if finished {
            finished = false;
            usleep(10 * 60 * 1_000_000);
            cur_link = sh.connect(ConnectionType::TCP, b"wttr.in", 80).unwrap();
            writeln!(console, "∙ \x1b[38;5;141m[{}]\x1b[0m Opening TCP conn", cur_link).unwrap();
        }

        // Receive into buffer
        let mut lastrecv = mcycle::read();
        while ofs < serial_buf.len() {
            // Read until we stop receiving for a certain duration
            // This is a hack around the fact that in the time that the parser runs,
            // more than one FIFO full of characters can be received so characters could be
            // lost. The right way would be to receive in an interrupt handler, but,
            // we don't have that yet.
            if let Ok(ch) = wrx.read() {
                serial_buf[ofs] = ch;
                ofs += 1;
                lastrecv = mcycle::read();
            } else if (mcycle::read().wrapping_sub(lastrecv)) >= TIMEOUT {
                break;
            }
        }
        //writeln!(debug, "ofs: {} received {} chars {:?}", ofs0, ofs - ofs0,
        //         &serial_buf[ofs0..ofs]).unwrap();

        // Loop as long as there's something in the buffer to parse, starting at the
        // beginning
        let mut start = 0;
        while start < ofs {
            // try parsing
            let tail = &serial_buf[start..ofs];
            let erase = match parse(tail) {
                ParseResult::Ok(offset, resp) => {
                    sh.message(&resp, |port, ev, _debug| {
                        match ev {
                            NetworkEvent::Ready => {
                                writeln!(console, "∙ Connected to AP").unwrap();
                                cur_link = port.connect(ConnectionType::TCP, b"wttr.in", 80).unwrap();
                                writeln!(console, "∙ \x1b[38;5;141m[{}]\x1b[0m Opening TCP conn", cur_link).unwrap();
                            }
                            NetworkEvent::Error => {
                                writeln!(console, "∙ Could not connect to AP").unwrap();
                            }
                            NetworkEvent::ListenSuccess(ip, port) => {
                                writeln!(console, "∙ Listening on {}.{}.{}.{}:{}",
                                         ip[0], ip[1], ip[2], ip[3], port).unwrap();
                            }
                            NetworkEvent::ConnectionEstablished(link) => {
                                if link == cur_link {
                                    writeln!(console, "∙ \x1b[38;5;141m[{}]\x1b[0m Sending HTTP request", link).unwrap();
                                    port.write_all(b"GET /?0qA HTTP/1.1\r\nHost: wttr.in\r\nConnection: close\r\nUser-Agent: Weather-Spy\r\n\r\n").unwrap();
                                    port.send(link).unwrap();
                                }
                            }
                            NetworkEvent::Data(link, data) => {
                                // write!(debug, "{}", str::from_utf8(data).unwrap());
                                if link == cur_link {
                                    console.puts(str::from_utf8(data).unwrap_or("???"));
                                }
                            }
                            NetworkEvent::ConnectionClosed(link) => {
                                writeln!(console, "∙ \x1b[38;5;141m[{}]\x1b[0m \x1b[38;2;100;100;100m[closed]\x1b[0m", link).unwrap();
                                finished = true;
                            }
                            _ => { }
                        }
                    }, &mut debug).unwrap();

                    offset
                }
                ParseResult::Incomplete => {
                    // Incomplete, ignored, just retry after a new receive
                    0
                }
                ParseResult::Err => {
                    if tail.len() > 100 {
                        writeln!(debug, "err: Error([too long ...])").unwrap();
                    } else {
                        writeln!(debug, "err: {:?}", tail).unwrap();
                    }
                    // Erase unparseable data to next line, if line is complete
                    if let Some(ofs) = tail.iter().position(|&x| x == b'\n') {
                        ofs + 1
                    } else {
                        // If not, retry next time
                        0
                    }
                }
            };

            if erase == 0 {
                // End of input or remainder unparseable
                break;
            }
            start += erase;
        }
        // Erase everything before new starting offset
        for i in start..ofs {
            serial_buf[i - start] = serial_buf[i];
        }
        ofs -= start;

        // If the buffer is full and we can't parse *anything*, clear it and start over
        if ofs == serial_buf.len() {
            writeln!(debug, "Error: buffer was unparseable, dropping buffer").unwrap();
            ofs = 0;
        }
    }
}
