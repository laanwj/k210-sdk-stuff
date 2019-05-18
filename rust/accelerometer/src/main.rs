#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![no_std]
#![no_main]

use k210_hal::pac;
use k210_hal::prelude::*;
use k210_hal::stdout::Stdout;
use k210_shared::board::def::{io,DISP_WIDTH,DISP_HEIGHT,MSA300_SLV_ADDR,MSA300_ADDR_BITS,MSA300_CLK};
use k210_shared::board::lcd::{LCD,self};
use k210_shared::board::lcd_colors;
use k210_shared::board::msa300::Accelerometer;
use k210_shared::soc::fpioa;
use k210_shared::soc::i2c::{I2C,I2CExt};
use k210_shared::soc::sleep::usleep;
use k210_shared::soc::spi::SPIExt;
use k210_shared::soc::sysctl;
use libm::F32Ext;
use riscv_rt::entry;

pub const BLK_SIZE: usize = 8;
pub const GRID_WIDTH: usize = DISP_WIDTH / BLK_SIZE;
pub const GRID_HEIGHT: usize = DISP_HEIGHT / BLK_SIZE;

/** Array for representing an image of the entire screen.
 * This is an array of DISP_WIDTH / 2 × DISP_HEIGHT, each two horizontally consecutive
 * pixels are encoded in a u32 with `(a << 16)|b`.
 */
pub type ScreenImage = [u32; DISP_WIDTH * DISP_HEIGHT / 2];

/** Connect pins to internal functions */
fn io_mux_init() {
    /* Init SPI IO map and function settings */
    fpioa::set_function(io::LCD_RST.into(), fpioa::function::gpiohs(lcd::RST_GPIONUM));
    fpioa::set_io_pull(io::LCD_RST.into(), fpioa::pull::DOWN); // outputs must be pull-down
    fpioa::set_function(io::LCD_DC.into(), fpioa::function::gpiohs(lcd::DCX_GPIONUM));
    fpioa::set_io_pull(io::LCD_DC.into(), fpioa::pull::DOWN);
    fpioa::set_function(io::LCD_CS.into(), fpioa::function::SPI0_SS3);
    fpioa::set_function(io::LCD_WR.into(), fpioa::function::SPI0_SCLK);

    /* I2C0 for touch-screen */
    fpioa::set_function(io::I2C1_SCL.into(), fpioa::function::I2C0_SCLK);
    fpioa::set_function(io::I2C1_SDA.into(), fpioa::function::I2C0_SDA);

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
    let p = pac::Peripherals::take().unwrap();

    let clocks = k210_hal::clock::Clocks::new();

    usleep(200000);

    // Configure UART
    let serial = p.UARTHS.constrain(115_200.bps(), &clocks);
    let (mut tx, _) = serial.split();

    let mut stdout = Stdout(&mut tx);

    io_mux_init();
    io_set_power();

    let spi = p.SPI0.constrain();
    let lcd = LCD::new(spi);
    lcd.init();
    lcd.set_direction(lcd::direction::YX_LRUD);
    lcd.clear(lcd_colors::PURPLE);

    let mut image: ScreenImage = [0; DISP_WIDTH * DISP_HEIGHT / 2];

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
        let mut idx = 0;
        for y in 0..DISP_HEIGHT as i32 {
            for x2 in 0..(DISP_WIDTH/2) as i32 {
                let x = x2 * 2;
                image[idx] = (if sample_cirle(x + 0, y, cx, cy, r, rr) { 0xffff } else { 0 } << 16) |
                             if sample_cirle(x + 1, y, cx, cy, r, rr) { 0xffff } else { 0 };
                idx += 1;
            }
        }

        lcd.draw_picture(0, 0, DISP_WIDTH as u16, DISP_HEIGHT as u16, &image);
        // usleep(10000);
    }
}
