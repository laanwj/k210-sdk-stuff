#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![no_std]
#![no_main]

use k210_console::console::{Console, ScreenImage};
use k210_console::cp437;
use k210_console::cp437_8x8::{FONT, GLYPH_BY_FILL};
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

/** 64-byte aligned planar RAM */
#[repr(C, align(64))]
struct PlanarScreenRAM {
    pub r: [u8; DISP_PIXELS],
    pub g: [u8; DISP_PIXELS],
    pub b: [u8; DISP_PIXELS],
}
impl PlanarScreenRAM {
    fn as_mut_ptrs(&mut self) -> (*mut u8, *mut u8, *mut u8) {
        (self.r.as_mut_ptr(),self.g.as_mut_ptr(),self.b.as_mut_ptr())
    }
}

static mut FRAME_AI: PlanarScreenRAM = PlanarScreenRAM {
    r: [0; DISP_PIXELS],
    g: [0; DISP_PIXELS],
    b: [0; DISP_PIXELS],
};

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
    lcd.set_direction(lcd::direction::YX_LRUD);
    lcd.clear(lcd_colors::PURPLE);

    let mut dvp = p.DVP.constrain();
    writeln!(stdout, "OV2640: init").unwrap();
    dvp.init(sccb_addr_len::W8);
    dvp.set_xclk_rate(24000000);
    dvp.set_image_format(image_format::RGB);
    dvp.set_image_size(true, 320, 240);
    ov2640::init(&dvp);
    writeln!(stdout, "OV2640: init done").unwrap();

    // use planar output for convenient sampling
    dvp.set_ai_addr(Some(unsafe { FRAME_AI.as_mut_ptrs() }));
    dvp.set_display_addr(None);
    dvp.set_auto(false);

    let mut image: ScreenImage = [0; DISP_PIXELS / 2];
    let mut console: Console = Console::new(&cp437::to, &FONT, None);
    writeln!(stdout, "Starting frame loop").unwrap();
    loop {
        dvp.get_image();

        for y in 0..console.height() {
            for x in 0..console.width() {
                // compute average over cell: could use some kernel that emphasizes the center
                // but this works fairly okay and is nice and fast
                // /
                // need to mirror x and y here so that characters are right side up
                // compared to camera image
                let mut r = 0;
                let mut g = 0;
                let mut b = 0;
                for iy in 0..8 {
                    for ix in 0..8 {
                        let cx = 319 - (usize::from(x) * 8 + ix);
                        let cy = 239 - (usize::from(y) * 8 + iy);
                        let addr = cy * 320 + cx;
                        r += unsafe { FRAME_AI.r[addr] } as u32;
                        g += unsafe { FRAME_AI.g[addr] } as u32;
                        b += unsafe { FRAME_AI.b[addr] } as u32;
                    }
                }
                r /= 8*8;
                g /= 8*8;
                b /= 8*8;
                let i = (77*r + 150*g + 29*b) / 256;
                console.put_raw(
                    x, y,
                    lcd_colors::rgb565(r as u8, g as u8, b as u8),
                    0,
                    GLYPH_BY_FILL[i as usize].into(),
                    0,
                );
            }
        }

        console.render(&mut image);
        lcd.draw_picture(0, 0, DISP_WIDTH, DISP_HEIGHT, &image);
    }
}
