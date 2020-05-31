/*! Stream raw data from SD card to LCD, bascially a test for using both SPI0 and SPI1 
 * at the same time
 */
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![no_std]
#![no_main]

use core::convert::TryInto;
use k210_hal::prelude::*;
use k210_hal::stdout::Stdout;
use k210_hal::pac::Peripherals;
use k210_shared::board::def::{io, DISP_HEIGHT, DISP_PIXELS, DISP_WIDTH};
use k210_shared::board::lcd::{self, LCD, LCDHL};
use k210_shared::board::lcd_colors;
use k210_shared::board::sdcard;
use k210_shared::soc::dmac::{dma_channel, DMACExt};
use k210_shared::soc::fpioa;
use k210_shared::soc::sleep::usleep;
use k210_shared::soc::spi::SPIExt;
use k210_shared::soc::sysctl;
use riscv_rt::entry;

/** GPIOHS GPIO number to use for controlling the SD card CS pin */
const SD_CS_GPIONUM: u8 = 7;
/** CS value passed to SPI controller, this is a dummy value as SPI0_CS3 is not mapped to anything
 * in the FPIOA */
const SD_CS: u32 = 3;

pub type ScreenImage = [u32; DISP_PIXELS / 2];

/** Connect pins to internal functions */
fn io_init() {
    /* Init SD card function settings */
    fpioa::set_function(io::SPI0_SCLK, fpioa::function::SPI1_SCLK);
    fpioa::set_function(io::SPI0_MOSI, fpioa::function::SPI1_D0);
    fpioa::set_function(io::SPI0_MISO, fpioa::function::SPI1_D1);
    fpioa::set_function(io::SPI0_CS0, fpioa::function::gpiohs(SD_CS_GPIONUM));
    fpioa::set_io_pull(io::SPI0_CS0, fpioa::pull::DOWN); // GPIO output=pull down

    /* Init LCD function settings */
    fpioa::set_function(io::LCD_RST, fpioa::function::gpiohs(lcd::RST_GPIONUM));
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

    // Configure UART
    let serial = p
        .UARTHS
        .configure(115_200.bps(), &clocks);
    let (mut tx, _) = serial.split();

    let mut stdout = Stdout(&mut tx);

    io_init();

    let dmac = p.DMAC.configure();

    let lspi = p.SPI0.constrain();
    let mut lcd = LCD::new(lspi, &dmac, dma_channel::CHANNEL0);
    lcd.init();
    lcd.set_direction(lcd::direction::YX_LRUD);
    lcd.clear(lcd_colors::PURPLE);

    let sspi = p.SPI1.constrain();
    writeln!(stdout, "sdcard: pre-init").unwrap();
    let sd = sdcard::SDCard::new(sspi, SD_CS, SD_CS_GPIONUM, &dmac, dma_channel::CHANNEL1);
    let info = sd.init().unwrap();
    writeln!(stdout, "card info: {:?}", info).unwrap();
    let num_sectors = info.CardCapacity / 512;
    writeln!(stdout, "number of sectors on card: {}", num_sectors).unwrap();

    assert!(num_sectors > 0);
    let mut sector: u64 = 0;
    let mut image: ScreenImage = [0; DISP_PIXELS / 2];
    let mut buffer = [0u8; DISP_PIXELS * 2];
    while sector < num_sectors {
        /* Read raw image */
        sd.read_sector(&mut buffer, sector.try_into().unwrap())
            .unwrap();
        writeln!(stdout, "sector {} succesfully read", sector).unwrap();
        let mut i = buffer.iter();
        /* Combine into u32s, reordering 16-bit halves */
        for x in image.iter_mut() {
            *x = (u32::from(*i.next().unwrap()) << 16)
                | (u32::from(*i.next().unwrap()) << 24)
                | (u32::from(*i.next().unwrap()) << 0)
                | (u32::from(*i.next().unwrap()) << 8);
        }
        lcd.draw_picture(0, 0, DISP_WIDTH, DISP_HEIGHT, &image);

        sector += (buffer.len() / 512) as u64;
    }
    loop {}
}
