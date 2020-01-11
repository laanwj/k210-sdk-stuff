/*! Voxel renderer.
 * Based on Sebastian Macke's excellent examples in https://github.com/s-macke/VoxelSpace
 */
#![no_std]
#![no_main]
use libm::F32Ext;

use k210_hal::prelude::*;
use k210_hal::stdout::Stdout;
use k210_hal::Peripherals;
use k210_shared::board::def::{io, DISP_HEIGHT, DISP_PIXELS, DISP_WIDTH};
use k210_shared::board::lcd::{self, LCD, LCDHL};
use k210_shared::board::lcd_colors;
use k210_shared::board::lcd_render::ScreenImage;
use k210_shared::soc::dmac::{dma_channel, DMACExt};
use k210_shared::soc::fpioa;
use k210_shared::soc::sleep::usleep;
use k210_shared::soc::spi::SPIExt;
use k210_shared::soc::sysctl;
use riscv_rt::entry;

mod map_data;

/** Euclidian modulus. */
pub fn mod_euc(a: i32, b: i32) -> i32 {
    let r = a % b;
    if r < 0 {
        r + b
    } else {
        r
    }
}

/** Minimum of two f32 values. */
#[allow(dead_code)]
fn fmin(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}
/** Maximum of two f32 values. */
fn fmax(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

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

/** R5G6B65 color value. */
pub type Color = u16;

/** In-memory voxel map. */
struct VoxelMap {
    pub width: u32,
    pub height: u32,
    pub palette: &'static [u16],
    pub data: &'static [u16],
}

impl VoxelMap {
    /** Input consists of a 8-bit palette (256 * u16 R5G6B5) followed by two raw interleaved images of size width
     * by height (width * height * D8P8), as generated by the "convert.py" conversion script.
     */
    pub fn new(width: u32, height: u32, data: &'static [u8]) -> Self {
        let data =
            unsafe { core::slice::from_raw_parts_mut(data.as_ptr() as *mut u16, data.len() / 2) };
        Self {
            width,
            height,
            palette: &data[0..256],
            data: &data[256..],
        }
    }

    /** Sample color and depth at coordinates x,y
     * - wrap around repeat x and y
     */
    pub fn sample(&self, x: f32, y: f32) -> (Color, u8) {
        // Get into 0..width / height range
        // TODO: this doesn't work due to lack of fmodf on the platform
        // let x = x % (self.width as f32);
        // let y = y % (self.height as f32);
        let x = mod_euc(x as i32, self.width as i32);
        let y = mod_euc(y as i32, self.height as i32);
        // Unpack value
        let ofs = y as usize * (self.width as usize) + x as usize;
        let val = self.data[ofs];
        (self.palette[(val & 0xff) as usize], (val >> 8) as u8)
    }
}

/** Display image in directly DMA'able u32 per two pixels
 * format.
 */
struct Display {
    pub data: ScreenImage,
}

/** Target for voxel-based rendering. This needs only one operation: draw
 * a vertical line.
 */
trait VoxelTarget {
    /** Draw a vertical line at x coordinate `cx`, from y coordinate `cy1` to `cy2`. */
    fn dvline(&mut self, cx: i32, cy1: i32, cy2: i32, color: Color);
}

impl Display {
    pub fn new() -> Self {
        Self {
            data: [0; DISP_PIXELS / 2],
        }
    }

    /** Image data as mutable [u16] for internal drawing use. */
    fn data(&mut self) -> &mut [u16] {
        unsafe { core::slice::from_raw_parts_mut(self.data.as_ptr() as *mut u16, DISP_PIXELS) }
    }
}

impl VoxelTarget for Display {
    /** Draw a vertical line at x coordinate `cx`, from y coordinate `cy1` to `cy2`. */
    fn dvline(&mut self, cx: i32, cy1: i32, cy2: i32, color: Color) {
        let data = self.data();
        if cx < 0 || cx >= (DISP_WIDTH as i32) {
            return;
        }
        let xofs = (cx ^ 1) as usize;
        for y in cy1..cy2 {
            if y >= 0 && y < (DISP_HEIGHT as i32) {
                data[(y as usize) * (DISP_WIDTH as usize) + xofs] = color;
            }
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

    writeln!(stdout, "Init DMAC").unwrap();
    let dmac = p.DMAC.configure();
    let chan = dma_channel::CHANNEL0;
    writeln!(
        stdout,
        "DMAC: id 0x{:x} version 0x{:x} AXI ID 0x{:x}",
        dmac.read_id(),
        dmac.read_version(),
        dmac.read_channel_id(chan)
    )
    .unwrap();

    let map = VoxelMap::new(map_data::WIDTH, map_data::HEIGHT, map_data::VOXEL_MAP);

    let spi = p.SPI0.constrain();
    let mut lcd = LCD::new(spi, &dmac, chan);
    lcd.init();
    lcd.set_direction(lcd::direction::YX_LRUD);
    lcd.clear(lcd_colors::PURPLE);

    // Some renderer constants:
    let sky_color = lcd_colors::rgb565(50, 100, 200); // Color of sky
    let horizon = (DISP_HEIGHT / 2) as f32; // y coordinate of horizon on screen
    let scale_height = 50.0; // Scaling factor for mountain heights
    let distance = 256; // Rendering distance

    writeln!(stdout, "First frame").unwrap();
    let mut disp = Display::new();
    // "Player" position
    let mut p = (0.0, 0.0);
    // "Player" rotation
    let mut phi = 0.0f32;
    loop {
        // Orientation to variables for 2D rotation matrix.
        let sinphi = phi.sin();
        let cosphi = phi.cos();
        // Derive current player height from landscape height at the current
        // position.
        // XXX: it'd be nice to apply some kind of low-pass filter here
        // to prevent ugly sudden jumps, while not lying into mountains.
        let (_, p_depth) = map.sample(p.0, p.1);
        let height = fmax(p_depth as f32 + 20.0, 50.0);
        // Render landscape!
        let mut ybuffer = [DISP_HEIGHT as i32; DISP_WIDTH as usize];
        for z in 1..distance {
            // Compute both end-points of (rotated) line segment
            // that represents this horizontal display line on map.
            let z = z as f32;
            let mut pleft = (
                (-cosphi * z - sinphi * z) + p.0,
                (sinphi * z - cosphi * z) + p.1,
            );
            let pright = (
                (cosphi * z - sinphi * z) + p.0,
                (-sinphi * z - cosphi * z) + p.1,
            );
            // Compute step taken on map for each pixel in the x direction.
            let delta = (
                (pright.0 - pleft.0) / DISP_WIDTH as f32,
                (pright.1 - pleft.1) / DISP_WIDTH as f32,
            );
            // Perspective scaling for this distance, given height scaling.
            let rscale = scale_height / z;

            // Traverse the line segment
            for i in 0..DISP_WIDTH as i32 {
                let (color, depth) = map.sample(pleft.0, pleft.1);
                // Perform perspective projection for height on screen, taking into account
                // player height and horizon.
                let height_on_screen = (height - depth as f32) * rscale + horizon;
                let height_on_screen = height_on_screen as i32;
                // Clamp against y-buffer to make sure more distant landscape doesn't
                // render over closer parts that have already been drawn.
                if height_on_screen < ybuffer[i as usize] {
                    disp.dvline(i, height_on_screen, ybuffer[i as usize], color);
                    ybuffer[i as usize] = height_on_screen;
                }
                // Advance.
                pleft.0 += delta.0;
                pleft.1 += delta.1;
            }
        }

        // Fill the remainder of the display with the sky color.
        for i in 0..DISP_WIDTH as i32 {
            disp.dvline(i, 0, ybuffer[i as usize], sky_color);
        }

        lcd.draw_picture(0, 0, DISP_WIDTH, DISP_HEIGHT, &disp.data);

        p.1 -= 1.0;
        phi += 0.005;
    }
}
