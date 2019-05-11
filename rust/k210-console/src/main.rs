#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![no_std]
#![no_main]

extern crate panic_halt;

mod console;
mod cp437;
mod cp437_8x8;
mod lfsr;
mod palette_xterm256;

use k210_hal::pac;
use k210_hal::prelude::*;
use k210_hal::stdout::Stdout;
use k210_shared::board::lcd;
use k210_shared::board::lcd_colors;
use k210_shared::soc::fpioa;
use k210_shared::soc::sleep::usleep;
use k210_shared::soc::sysctl;
use riscv_rt::entry;

use crate::console::{Color, Console, ScreenImage, DISP_HEIGHT, DISP_WIDTH};
use crate::palette_xterm256::PALETTE;

/** Connect pins to internal functions */
fn io_mux_init() {
    /* Init SPI IO map and function settings */
    fpioa::set_function(37, fpioa::function::gpiohs(lcd::RST_GPIONUM));
    fpioa::set_io_pull(37, fpioa::pull::DOWN); // outputs must be pull-down
    fpioa::set_function(38, fpioa::function::gpiohs(lcd::DCX_GPIONUM));
    fpioa::set_io_pull(38, fpioa::pull::DOWN);
    fpioa::set_function(36, fpioa::function::SPI0_SS3);
    fpioa::set_function(39, fpioa::function::SPI0_SCLK);

    sysctl::set_spi0_dvp_data(true);
}

/** Set correct voltage for pins */
fn io_set_power() {
    /* Set dvp and spi pin to 1.8V */
    sysctl::set_power_mode(sysctl::power_bank::BANK6, sysctl::io_power_mode::V18);
    sysctl::set_power_mode(sysctl::power_bank::BANK7, sysctl::io_power_mode::V18);
}

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    // Configure clocks (TODO)
    let clocks = k210_hal::clock::Clocks::new();

    // sleep a bit to let clients connect
    usleep(200000);

    // Configure UART
    let serial = p.UARTHS.constrain(115_200.bps(), &clocks);
    let (mut tx, _) = serial.split();

    let mut stdout = Stdout(&mut tx);

    io_mux_init();
    io_set_power();

    writeln!(stdout, "Clocks:").unwrap();
    writeln!(
        stdout,
        "  CPU {}",
        sysctl::clock_get_freq(sysctl::clock::CPU)
    )
    .unwrap();
    writeln!(
        stdout,
        "  SPI0 {}",
        sysctl::clock_get_freq(sysctl::clock::SPI0)
    )
    .unwrap();
    writeln!(
        stdout,
        "  PLL0 {}",
        sysctl::clock_get_freq(sysctl::clock::PLL0)
    )
    .unwrap();
    writeln!(
        stdout,
        "  PLL1 {}",
        sysctl::clock_get_freq(sysctl::clock::PLL1)
    )
    .unwrap();
    writeln!(
        stdout,
        "  PLL2 {}",
        sysctl::clock_get_freq(sysctl::clock::PLL2)
    )
    .unwrap();

    /* LCD init */
    lcd::init();
    lcd::set_direction(lcd::direction::YX_RLDU);
    lcd::clear(lcd_colors::PURPLE);

    let mut image: ScreenImage = [0; DISP_WIDTH * DISP_HEIGHT / 2];
    let mut console: Console = Console::new();

    /* Make a border */
    let fg = Color::new(0x40, 0x40, 0x40);
    let bg = Color::new(0x00, 0x00, 0x00);
    // Sides
    for x in 1..console.width() - 1 {
        console.put(x, 0, fg, bg, cp437::from(196));
        console.put(x, console.height() - 1, fg, bg, cp437::from(196));
    }
    for y in 1..console.height() - 1 {
        console.put(0, y, fg, bg, cp437::from(179));
        console.put(console.width() - 1, y, fg, bg, cp437::from(179));
    }
    // Corners
    console.put(0, 0, fg, bg, cp437::from(218));
    console.put(console.width() - 1, 0, fg, bg, cp437::from(191));
    console.put(0, console.height() - 1, fg, bg, cp437::from(192));
    console.put(
        console.width() - 1,
        console.height() - 1,
        fg,
        bg,
        cp437::from(217),
    );

    let mut frame = 0;
    let mut s = lfsr::LFSR::new();
    loop {
        console.put(
            2,
            0,
            Color::new(0xc0, 0x00, 0xff),
            bg,
            ((0x30 + ((frame / 1000) % 10)) as u8) as char,
        );
        console.put(
            3,
            0,
            Color::new(0xd0, 0x00, 0xff),
            bg,
            ((0x30 + ((frame / 100) % 10)) as u8) as char,
        );
        console.put(
            4,
            0,
            Color::new(0xe0, 0x00, 0xff),
            bg,
            ((0x30 + ((frame / 10) % 10)) as u8) as char,
        );
        console.put(
            5,
            0,
            Color::new(0xf0, 0x00, 0xff),
            bg,
            ((0x30 + ((frame / 1) % 10)) as u8) as char,
        );

        /* just put some random stuff */
        for y in 2..console.height() - 2 {
            for x in 2..console.width() - 2 {
                let rv = s.next();
                console.put(
                    x,
                    y,
                    Color::from_rgba32(PALETTE[((rv >> 24) & 0xf) as usize]),
                    Color::from_rgba32(PALETTE[((rv >> 20) & 0x7) as usize]),
                    cp437::from((rv & 0xff) as u8),
                );
            }
        }

        console.render(&mut image);
        lcd::draw_picture(0, 0, DISP_WIDTH as u16, DISP_HEIGHT as u16, &image);

        writeln!(stdout, "test {}", frame).unwrap();
        usleep(1_000_000);
        frame += 1;
    }
}
