#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![no_std]
#![no_main]

use core::str;
use esp8266at::handler::{NetworkEvent, SerialNetworkHandler};
use esp8266at::response::{parse, ParseResult};
use esp8266at::traits;
use k210_hal::pac::Peripherals;
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
use riscv_rt::entry;
use k210_console::console::{Console, ScreenImage, DISP_HEIGHT, DISP_WIDTH, DISP_PIXELS};
use k210_console::{cp437, cp437_8x8};
use buffered_uart;

mod config;

const DEFAULT_BAUD: u32 = 115_200;

struct WriteAdapter;

impl WriteAdapter {
    fn new() -> Self {
        Self { }
    }
}
impl traits::Write for WriteAdapter {
    type Error = ();

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        buffered_uart::send(buf);
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
    sysctl::pll_set_freq(sysctl::pll::PLL0, 800_000_000).unwrap();
    sysctl::pll_set_freq(sysctl::pll::PLL1, 300_000_000).unwrap();
    sysctl::pll_set_freq(sysctl::pll::PLL2, 45_158_400).unwrap();
    let clocks = k210_hal::clock::Clocks::new();

    usleep(200000);
    io_init();

    // Configure UARTHS (→host)
    let serial = p.UARTHS.configure(DEFAULT_BAUD.bps(), &clocks);
    let (mut tx, mut _rx) = serial.split();
    let mut debug = Stdout(&mut tx);

    // Configure UART1 (→WIFI)
    sysctl::clock_enable(sysctl::clock::UART1);
    sysctl::reset(sysctl::reset::UART1);
    fpioa::set_function(io::WIFI_RX, fpioa::function::UART1_TX);
    fpioa::set_function(io::WIFI_TX, fpioa::function::UART1_RX);
    fpioa::set_function(io::WIFI_EN, fpioa::function::GPIOHS8);
    fpioa::set_io_pull(io::WIFI_EN, fpioa::pull::DOWN);
    gpiohs::set_pin(8, true);
    gpiohs::set_direction(8, gpio::direction::OUTPUT);

    buffered_uart::init();
    let mut wa = WriteAdapter::new();
    let mut sh = SerialNetworkHandler::new(&mut wa, config::APNAME.as_bytes(), config::APPASS.as_bytes());

    // LCD ini
    let dmac = p.DMAC.configure();
    let spi = p.SPI0.constrain();
    let mut lcd = LCD::new(spi, &dmac, dma_channel::CHANNEL0);
    lcd.init();
    lcd.set_direction(lcd::direction::YX_LRUD);
    let mut console: Console = Console::new(&cp437::to, &cp437_8x8::FONT, None);

    writeln!(console, "\x1b[48;2;128;192;255;38;5;0m TERMINAL \x1b[0m \x1b[38;2;128;128;128m\x1b[0m").unwrap();

    // Start off connection process state machine
    sh.start(false).unwrap();
    writeln!(console, "∙ Connecting to AP").unwrap();

    let mut serial_buf = [0u8; 3000]; // needs to accomodate one whole response which is 2*TCP MSS(=2920)+some
    let mut ofs: usize = 0;

    loop {
        if console.dirty {
            let mut image: ScreenImage = [0; DISP_PIXELS / 2];
            console.render(&mut image);
            lcd.draw_picture(0, 0, DISP_WIDTH, DISP_HEIGHT, &image);
            console.dirty = false;
        }

        // Receive into buffer
        ofs += buffered_uart::recv(&mut serial_buf[ofs..]);
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
                                port.listen(33445).unwrap();
                            }
                            NetworkEvent::Error => {
                                writeln!(console, "∙ Could not connect to AP").unwrap();
                            }
                            NetworkEvent::ListenSuccess(ip, port) => {
                                writeln!(console, "∙ Listening on {}.{}.{}.{}:{}",
                                         ip[0], ip[1], ip[2], ip[3], port).unwrap();
                            }
                            NetworkEvent::ConnectionEstablished(_link) => {
                            }
                            NetworkEvent::Data(_link, data) => {
                                // write!(debug, "{}", str::from_utf8(data).unwrap());
                                console.puts(str::from_utf8(data).unwrap_or("???"));
                            }
                            NetworkEvent::ConnectionClosed(_link) => {
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

        /*
        if let Ok(ch) = rx.read() {
            let _res = block!(wtx.write(ch));
        }
        */
    }
}
