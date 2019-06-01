#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![no_std]
#![no_main]

use k210_hal::pac;
use k210_hal::prelude::*;
use k210_hal::stdout::Stdout;
use k210_shared::board::def::{io,DISP_WIDTH,DISP_HEIGHT};
use k210_shared::board::lcd::{LCD,LCDHL,self};
use k210_shared::board::lcd_colors;
use k210_shared::board::lcd_render::render_image;
use k210_shared::soc::fpioa;
use k210_shared::soc::sleep::usleep;
use k210_shared::soc::spi::SPIExt;
use k210_shared::soc::sysctl;
use riscv_rt::entry;
use k210_shared::soc::dvp::{DVPExt,sccb_addr_len,image_format};
use k210_shared::board::ov2640;

pub type ScreenImage = [u32; DISP_WIDTH * DISP_HEIGHT / 2];

/** Connect pins to internal functions */
fn io_init() {
    /* Init DVP IO map and function settings */
    fpioa::set_function(io::DVP_RST.into(), fpioa::function::CMOS_RST);
    fpioa::set_function(io::DVP_PWDN.into(), fpioa::function::CMOS_PWDN);
    fpioa::set_function(io::DVP_XCLK.into(), fpioa::function::CMOS_XCLK);
    fpioa::set_function(io::DVP_VSYNC.into(), fpioa::function::CMOS_VSYNC);
    fpioa::set_function(io::DVP_HSYNC.into(), fpioa::function::CMOS_HREF);
    fpioa::set_function(io::DVP_PCLK.into(), fpioa::function::CMOS_PCLK);
    fpioa::set_function(io::DVP_SCL.into(), fpioa::function::SCCB_SCLK);
    fpioa::set_function(io::DVP_SDA.into(), fpioa::function::SCCB_SDA);

    /* Init SPI IO map and function settings */
    fpioa::set_function(io::LCD_RST.into(), fpioa::function::gpiohs(lcd::RST_GPIONUM));
    fpioa::set_io_pull(io::LCD_RST.into(), fpioa::pull::DOWN); // outputs must be pull-down
    fpioa::set_function(io::LCD_DC.into(), fpioa::function::gpiohs(lcd::DCX_GPIONUM));
    fpioa::set_io_pull(io::LCD_DC.into(), fpioa::pull::DOWN);
    fpioa::set_function(io::LCD_CS.into(), fpioa::function::SPI0_SS3);
    fpioa::set_function(io::LCD_WR.into(), fpioa::function::SPI0_SCLK);

    sysctl::set_spi0_dvp_data(true);

    /* Set DVP and SPI pin to 1.8V */
    sysctl::set_power_mode(sysctl::power_bank::BANK6, sysctl::io_power_mode::V18);
    sysctl::set_power_mode(sysctl::power_bank::BANK7, sysctl::io_power_mode::V18);
}

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    // Configure clocks (TODO)
    let clocks = k210_hal::clock::Clocks::new();

    usleep(200000);

    // Configure UART
    let serial = p.UARTHS.constrain(115_200.bps(), &clocks);
    let (mut tx, _) = serial.split();

    let mut stdout = Stdout(&mut tx);

    io_init();

    let spi = p.SPI0.constrain();
    let mut lcd = LCD::new(spi);
    lcd.init();
    lcd.set_direction(lcd::direction::YX_LRUD);
    lcd.clear(lcd_colors::PURPLE);

    let mut dvp = p.DVP.constrain();
    writeln!(stdout, "OV2640: init").unwrap();
    dvp.init(sccb_addr_len::W8);
    writeln!(stdout, "OV2640: set xclk rate").unwrap();
    dvp.set_xclk_rate(24000000);
    writeln!(stdout, "OV2640: set image format").unwrap();
    dvp.set_image_format(image_format::RGB);
    writeln!(stdout, "OV2640: set image size").unwrap();
    dvp.set_image_size(true, 320, 240);
    let (manuf_id, device_id) = ov2640::read_id(&dvp);
    writeln!(stdout, "OV2640: manuf 0x{:04x} device 0x{:04x}", manuf_id, device_id).unwrap();
    if manuf_id != 0x7fa2 || device_id != 0x2642 {
        writeln!(stdout, "Warning: unknown chip").unwrap();
    }
    ov2640::init(&dvp);
    writeln!(stdout, "OV2640: init done").unwrap();

    let mut image: ScreenImage = [0; DISP_WIDTH * DISP_HEIGHT / 2];

    dvp.set_ai_addr(None);
    dvp.set_display_addr(Some(image.as_mut_ptr()));
    dvp.set_auto(false);

    writeln!(stdout, "OV2640: starting frame loop").unwrap();
    loop {
        dvp.get_image();
        lcd.draw_picture(0, 0, DISP_WIDTH as u16, DISP_HEIGHT as u16, &image);
    }
}
