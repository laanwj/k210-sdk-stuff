#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![no_std]
#![no_main]

use k210_hal::pac::Peripherals;
use k210_hal::prelude::*;
use k210_hal::stdout::Stdout;
use k210_shared::board::def::{io,DISP_WIDTH,DISP_HEIGHT,DISP_PIXELS};
use k210_shared::board::lcd::{LCD,LCDHL,self};
use k210_shared::board::lcd_colors;
use k210_shared::soc::dmac::{DMACExt, dma_channel};
use k210_shared::soc::fpioa;
use k210_shared::soc::sleep::usleep;
use k210_shared::soc::spi::SPIExt;
use k210_shared::soc::sysctl;
use riscv_rt::entry;
use k210_shared::soc::dvp::{DVPExt,sccb_addr_len,image_format};
use k210_shared::board::ov2640;

/** 64-byte aligned screen RAM */
#[repr(C, align(64))]
struct ScreenRAM {
    pub image: [u32; DISP_PIXELS / 2],
}
impl ScreenRAM {
    fn as_mut_ptr(&mut self) -> *mut u32 { self.image.as_mut_ptr() }
}

static mut FRAME: ScreenRAM = ScreenRAM { image: [0; DISP_PIXELS / 2] };

/** Connect pins to internal functions */
fn io_init() {
    /* Init DVP IO map and function settings */
    fpioa::set_function(io::DVP_RST, fpioa::function::CMOS_RST);
    fpioa::set_function(io::DVP_PWDN, fpioa::function::CMOS_PWDN);
    fpioa::set_function(io::DVP_XCLK, fpioa::function::CMOS_XCLK);
    fpioa::set_function(io::DVP_VSYNC, fpioa::function::CMOS_VSYNC);
    fpioa::set_function(io::DVP_HSYNC, fpioa::function::CMOS_HREF);
    fpioa::set_function(io::DVP_PCLK, fpioa::function::CMOS_PCLK);
    fpioa::set_function(io::DVP_SCL, fpioa::function::SCCB_SCLK);
    fpioa::set_function(io::DVP_SDA, fpioa::function::SCCB_SDA);

    /* Init SPI IO map and function settings */
    fpioa::set_function(io::LCD_RST, fpioa::function::gpiohs(lcd::RST_GPIONUM));
    fpioa::set_io_pull(io::LCD_RST, fpioa::pull::DOWN); // outputs must be pull-down
    fpioa::set_function(io::LCD_DC, fpioa::function::gpiohs(lcd::DCX_GPIONUM));
    fpioa::set_io_pull(io::LCD_DC, fpioa::pull::DOWN);
    fpioa::set_function(io::LCD_CS, fpioa::function::SPI0_SS3);
    fpioa::set_function(io::LCD_WR, fpioa::function::SPI0_SCLK);

    sysctl::set_spi0_dvp_data(true);

    /* Set DVP and SPI pin to 1.8V */
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
    let serial = p.UARTHS.configure(115_200.bps(), &clocks);
    let (mut tx, _) = serial.split();

    let mut stdout = Stdout(&mut tx);

    io_init();

    let dmac = p.DMAC.configure();
    let spi = p.SPI0.constrain();
    let mut lcd = LCD::new(spi, &dmac, dma_channel::CHANNEL0);
    lcd.init();
    lcd.set_direction(lcd::direction::YX_RLDU);
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

    writeln!(stdout, "OV2640: setting display out addr to {:?}", unsafe { FRAME.as_mut_ptr() } ).unwrap();
    dvp.set_ai_addr(None);
    dvp.set_display_addr(Some(unsafe { FRAME.as_mut_ptr() }));
    dvp.set_auto(false);

    writeln!(stdout, "OV2640: starting frame loop").unwrap();
    loop {
        dvp.get_image();
        lcd.draw_picture(0, 0, DISP_WIDTH, DISP_HEIGHT, unsafe { &FRAME.image } );
    }
}
