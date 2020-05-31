#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![no_std]
#![no_main]

mod example_colorfont;
mod lfsr;

use k210_hal::prelude::*;
use k210_hal::stdout::Stdout;
use k210_hal::pac::Peripherals;
use k210_shared::board::def::io;
use k210_shared::board::lcd::{self, LCD, LCDHL};
use k210_shared::board::lcd_colors;
use k210_shared::soc::dmac::{dma_channel, DMACExt};
use k210_shared::soc::fpioa;
use k210_shared::soc::sleep::usleep;
use k210_shared::soc::spi::SPIExt;
use k210_shared::soc::sysctl;
use riscv_rt::entry;

use k210_console::console::{
    CellFlags, Color, Console, ScreenImage, DISP_HEIGHT, DISP_PIXELS, DISP_WIDTH,
};
use k210_console::palette_xterm256::PALETTE;
use k210_console::{cp437, cp437_8x8};

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

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();

    sysctl::pll_set_freq(sysctl::pll::PLL0, 800_000_000).unwrap();
    sysctl::pll_set_freq(sysctl::pll::PLL1, 300_000_000).unwrap();
    sysctl::pll_set_freq(sysctl::pll::PLL2, 45_158_400).unwrap();
    // Configure clocks (TODO)
    let clocks = k210_hal::clock::Clocks::new();

    // sleep a bit to let clients connect
    usleep(200000);

    // Configure UART
    let serial = p
        .UARTHS
        .configure(115_200.bps(), &clocks);
    let (mut tx, _) = serial.split();

    let mut stdout = Stdout(&mut tx);

    io_mux_init();
    io_set_power();

    writeln!(stdout, "Clocks:").unwrap();
    writeln!(
        stdout,
        "  CPU {} (HAL assumes {})",
        sysctl::clock_get_freq(sysctl::clock::CPU),
        clocks.cpu().0,
    )
    .unwrap();
    writeln!(
        stdout,
        "  KPU {}",
        sysctl::clock_get_freq(sysctl::clock::AI),
    )
    .unwrap();
    writeln!(
        stdout,
        "  APB0 {} (HAL assumes {})",
        sysctl::clock_get_freq(sysctl::clock::APB0),
        clocks.apb0().0,
    )
    .unwrap();
    writeln!(
        stdout,
        "  APB1 {}",
        sysctl::clock_get_freq(sysctl::clock::APB1)
    )
    .unwrap();
    writeln!(
        stdout,
        "  APB2 {}",
        sysctl::clock_get_freq(sysctl::clock::APB2)
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
    let dmac = p.DMAC.configure();
    let spi = p.SPI0.constrain();
    let mut lcd = LCD::new(spi, &dmac, dma_channel::CHANNEL0);
    lcd.init();
    lcd.set_direction(lcd::direction::YX_LRUD);
    lcd.clear(lcd_colors::PURPLE);

    let mut image: ScreenImage = [0; DISP_PIXELS / 2];
    let mut console: Console =
        Console::new(&cp437::to, &cp437_8x8::FONT, Some(&example_colorfont::FONT));

    /* Make a border */
    let fg = Color::new(0x40, 0x40, 0x40);
    let bg = Color::new(0x00, 0x00, 0x00);
    // Sides
    for x in 1..console.width() - 1 {
        console.put(x, 0, fg, bg, '─');
        console.put(x, console.height() - 1, fg, bg, '─');
    }
    for y in 1..console.height() - 1 {
        console.put(0, y, fg, bg, '│');
        console.put(console.width() - 1, y, fg, bg, '│');
    }
    // Corners
    console.put(0, 0, fg, bg, '┌');
    console.put(console.width() - 1, 0, fg, bg, '┐');
    console.put(0, console.height() - 1, fg, bg, '└');
    console.put(console.width() - 1, console.height() - 1, fg, bg, '┘');

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
        for y in 10..console.height() - 2 {
            for x in 2..console.width() - 2 {
                let rv = s.next();
                console.put(
                    x,
                    y,
                    Color::from_rgb565(PALETTE[((rv >> 24) & 0xf) as usize]),
                    Color::from_rgb565(PALETTE[((rv >> 20) & 0x7) as usize]),
                    cp437::from((rv & 0xff) as u8),
                );
            }
        }

        /* overlay image */
        for y in 0..7 {
            for x in 0..20 {
                console.put_raw(
                    x + 9,
                    y + 2,
                    0,
                    0,
                    example_colorfont::SEQ0[usize::from(y)][usize::from(x)],
                    CellFlags::COLOR,
                );
            }
        }

        console.render(&mut image);
        lcd.draw_picture(0, 0, DISP_WIDTH, DISP_HEIGHT, &image);

        // writeln!(stdout, "test {}", frame).unwrap();
        usleep(100_000);
        frame += 1;
    }
}
