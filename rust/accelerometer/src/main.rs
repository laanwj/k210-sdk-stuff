#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![no_std]
#![no_main]

use k210_hal::pac::Peripherals;
use k210_hal::prelude::*;
use k210_hal::stdout::Stdout;
use k210_shared::board::def::{io,DISP_WIDTH,DISP_HEIGHT,MSA300_SLV_ADDR,MSA300_ADDR_BITS,MSA300_CLK};
use k210_shared::board::lcd::{LCD,LCDHL,self};
use k210_shared::board::lcd_colors;
use k210_shared::board::lcd_render::render_image;
use k210_shared::board::msa300::Accelerometer;
use k210_shared::soc::dmac::{DMACExt, dma_channel};
use k210_shared::soc::fpioa;
use k210_shared::soc::i2c::{I2C,I2CExt};
use k210_shared::soc::sleep::usleep;
use k210_shared::soc::spi::SPIExt;
use k210_shared::soc::sysctl;
use libm::F32Ext;
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

    /* I2C0 for touch-screen */
    fpioa::set_function(io::I2C1_SCL, fpioa::function::I2C0_SCLK);
    fpioa::set_function(io::I2C1_SDA, fpioa::function::I2C0_SDA);

    sysctl::set_spi0_dvp_data(true);
}

/** Set correct voltage for pins */
fn io_set_power() {
    /* Set dvp and spi pin to 1.8V */
    sysctl::set_power_mode(sysctl::power_bank::BANK6, sysctl::io_power_mode::V18);
    sysctl::set_power_mode(sysctl::power_bank::BANK7, sysctl::io_power_mode::V18);
}

fn sample_cirle(x: i32, y: i32, cx: i32, cy: i32, r: i32, rr: i32) -> bool {
    // Early-out based on bounding box
    (x - cx).abs() <= r && (y - cy).abs() <= r &&
      ((x - cx) * (x - cx) +  (y - cy) * (y - cy)) <= rr
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
    let serial = p.UARTHS.configure(115_200.bps(), &clocks);
    let (mut tx, _) = serial.split();

    let mut stdout = Stdout(&mut tx);

    io_mux_init();
    io_set_power();

    let dmac = p.DMAC.configure();
    let spi = p.SPI0.constrain();
    let mut lcd = LCD::new(spi, &dmac, dma_channel::CHANNEL0);
    lcd.init();
    lcd.set_direction(lcd::direction::YX_LRUD);
    lcd.clear(lcd_colors::PURPLE);

    writeln!(stdout, "MSA300 init").unwrap();
    let i2c = p.I2C0.constrain();
    i2c.init(MSA300_SLV_ADDR, MSA300_ADDR_BITS, MSA300_CLK);
    let acc = Accelerometer::init(i2c).unwrap();

    loop {
        let (x, y, z) = acc.measure().unwrap();
        let mag = (x*x+y*y+z*z).sqrt();
        // writeln!(stdout, "m/s^2 x={} y={} z={} (size={})", x, y, z, mag).unwrap();

        // draw bubble
        let cx = ((y/8.5 + 1.0) * ((DISP_WIDTH / 2) as f32)) as i32;
        let cy = ((x/8.5 + 1.0) * ((DISP_HEIGHT / 2) as f32)) as i32;
        let r = (1.5*mag) as i32;
        let rr = r * r;
        render_image(&mut lcd, |x,y| {
            if sample_cirle(i32::from(x), i32::from(y), cx, cy, r, rr) {
                0xffff
            } else {
                0
            }
        });
        // usleep(10000);
    }
}
