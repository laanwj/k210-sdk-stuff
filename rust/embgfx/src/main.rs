#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![no_std]
#![no_main]

use embedded_graphics::fonts::Font6x8;
use embedded_graphics::image::ImageBmp;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Circle;
use embedded_graphics::Drawing;
use embedded_graphics::{icoord, text_6x8};
use k210_hal::prelude::*;
use k210_hal::stdout::Stdout;
use k210_hal::Peripherals;
use k210_shared::board::def::{io, DISP_HEIGHT, DISP_PIXELS, DISP_WIDTH};
use k210_shared::board::lcd::{self, LCD, LCDHL};
use k210_shared::board::lcd_render::ScreenImage;
use k210_shared::soc::dmac::{dma_channel, DMACExt};
use k210_shared::soc::fpioa;
use k210_shared::soc::sleep::usleep;
use k210_shared::soc::spi::SPIExt;
use k210_shared::soc::sysctl;
use riscv_rt::entry;

/** Connect pins to internal functions */
fn io_mux_init() {
    /* Init SPI IO map and function settings */
    fpioa::set_function(io::LCD_RST, fpioa::function::gpiohs(lcd::RST_GPIONUM));
    fpioa::set_io_pull(io::LCD_RST, fpioa::pull::DOWN); // outputs must be pull-down
    fpioa::set_function(io::LCD_DC, fpioa::function::gpiohs(lcd::DCX_GPIONUM));
    fpioa::set_io_pull(io::LCD_DC, fpioa::pull::DOWN);
    fpioa::set_function(io::LCD_CS, fpioa::function::SPI0_SS3);
    fpioa::set_function(io::LCD_WR, fpioa::function::SPI0_SCLK);

    sysctl::set_spi0_dvp_data(true);
}

/** Set correct voltage for pins */
fn io_set_power() {
    /* Set dvp and spi pin to 1.8V */
    sysctl::set_power_mode(sysctl::power_bank::BANK6, sysctl::io_power_mode::V18);
    sysctl::set_power_mode(sysctl::power_bank::BANK7, sysctl::io_power_mode::V18);
}

/** Implement embedded_graphics display traits for LCD.*/
struct Display {
    // Currently this only supports flushing the entire screen.
    // It's possible to improve on this by keeping track of which parts of the screen have been
    // updated, as the display does support partial (rectangle) updates.
    pub data: ScreenImage,
}

impl Display {
    fn new() -> Display {
        Display {
            data: [0; DISP_PIXELS / 2],
        }
    }

    fn flush<T: LCDHL>(&self, lcd: &T) {
        lcd.draw_picture(0, 0, DISP_WIDTH, DISP_HEIGHT, &self.data);
    }
}

impl Drawing<Rgb565> for Display {
    fn draw<T>(&mut self, item: T)
    where
        T: IntoIterator<Item = Pixel<Rgb565>>,
    {
        let data = unsafe {core::slice::from_raw_parts_mut(self.data.as_ptr() as *mut u16, DISP_PIXELS)};
        for Pixel(coord, color) in item {
            let index = (coord[0]^1) + (coord[1] * (DISP_WIDTH as u32));
            data[index as usize] = u16::from(color);
        }
    }
}

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();

    sysctl::pll_set_freq(sysctl::pll::PLL0, 800_000_000).unwrap();
    sysctl::pll_set_freq(sysctl::pll::PLL1, 300_000_000).unwrap();
    sysctl::pll_set_freq(sysctl::pll::PLL2, 45_158_400).unwrap();
    let clocks = k210_hal::clock::Clocks::new();

    usleep(200000);

    // Configure UART
    let serial = p
        .UARTHS
        .configure((p.pins.pin5, p.pins.pin4), 115_200.bps(), &clocks);
    let (mut tx, _) = serial.split();

    let mut stdout = Stdout(&mut tx);

    io_mux_init();
    io_set_power();

    writeln!(stdout, "First frame").unwrap();
    let dmac = p.DMAC.configure();
    let spi = p.SPI0.constrain();
    let mut lcd = LCD::new(spi, &dmac, dma_channel::CHANNEL0);
    lcd.init();
    lcd.set_direction(lcd::direction::YX_LRUD);

    let mut display = Display::new();

    let c = Circle::new(Coord::new(20, 20), 8).fill(Some(Rgb565::RED));
    let t = Font6x8::render_str("Hello Rust!")
        .fill(Some(Rgb565::GREEN))
        .translate(Coord::new(20, 16));
    let image: ImageBmp<Rgb565> = ImageBmp::new(include_bytes!("./rust-pride.bmp"))
        .unwrap()
        .translate(Coord::new(100, 20));

    loop {
        display.draw(c);
        display.draw(t);
        display.draw(&image);

        display.draw(
            text_6x8!("Hello world! - no background", stroke = Some(Rgb565::WHITE))
                .translate(icoord!(15, 115)),
        );

        display.draw(
            text_6x8!(
                "Hello world! - filled background",
                stroke = Some(Rgb565::YELLOW),
                fill = Some(Rgb565::BLUE)
            )
            .translate(icoord!(15, 130)),
        );

        display.draw(
            text_6x8!(
                "Hello world! - inverse background",
                stroke = Some(Rgb565::BLUE),
                fill = Some(Rgb565::YELLOW)
            )
            .translate(icoord!(15, 145)),
        );

        display.flush(&lcd);
    }
}
