#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![no_std]
#![no_main]

use k210_hal::pac::Peripherals;
use k210_hal::prelude::*;
use k210_hal::stdout::Stdout;
use k210_shared::board::def::{io,DISP_WIDTH,DISP_HEIGHT,DISP_PIXELS,NS2009_SLV_ADDR,NS2009_CAL,NS2009_ADDR_BITS,NS2009_CLK};
use k210_shared::board::lcd::{LCD,LCDHL,self};
use k210_shared::board::lcd_colors;
use k210_shared::board::ns2009::TouchScreen;
use k210_shared::soc::dmac::{DMACExt, dma_channel};
use k210_shared::soc::fpioa;
use k210_shared::soc::i2c::{I2C,I2CExt};
use k210_shared::soc::sleep::usleep;
use k210_shared::soc::spi::SPIExt;
use k210_shared::soc::sysctl;
use riscv_rt::entry;

pub const BLK_SIZE: usize = 8;
pub const GRID_WIDTH: usize = (DISP_WIDTH as usize) / BLK_SIZE;
pub const GRID_HEIGHT: usize = (DISP_HEIGHT as usize) / BLK_SIZE;

/** Array for representing an image of the entire screen.
 * This is an array of DISP_WIDTH / 2 × DISP_HEIGHT, each two horizontally consecutive
 * pixels are encoded in a u32 with `(a << 16)|b`.
 */
pub type ScreenImage = [u32; DISP_PIXELS / 2];

/** Universe abstraction */
struct Universe {
    state: [[bool; GRID_WIDTH*GRID_HEIGHT]; 2],
    cur: usize,
}

impl Universe {
    /** Create a new universe */
    pub fn new() -> Self {
        Self {
            state: [[false; GRID_WIDTH*GRID_HEIGHT]; 2],
            cur: 0,
        }
    }

    /** Get status of a cell */
    pub fn get(&self, x: usize, y: usize) -> bool {
        self.state[self.cur][y * GRID_WIDTH + x]
    }

    /** Set status of a cell */
    pub fn set(&mut self, x: usize, y: usize, state: bool) {
        self.state[self.cur][y * GRID_WIDTH + x] = state;
    }

    /** Toggle a cell dead/alive */
    pub fn toggle(&mut self, x: usize, y: usize) {
        self.state[self.cur][y * GRID_WIDTH + x] ^= true;
    }

    /** Run cellular automaton */
    pub fn iterate(&mut self) {
        for y in 0..GRID_HEIGHT {
            // wrap around y
            let ypi = if y == 0 { GRID_HEIGHT-1 } else { y-1 } * GRID_WIDTH;
            let yi = y * GRID_WIDTH;
            let yni = if y == GRID_HEIGHT-1 { 0 } else { y+1 } * GRID_WIDTH;
            for x in 0..GRID_WIDTH {
                // wrap around x
                let xp = if x == 0 { GRID_WIDTH-1 } else { x-1 };
                let xn = if x == GRID_WIDTH-1 { 0 } else { x+1 };

                let count = u32::from(self.state[self.cur][ypi + xp]) +
                            u32::from(self.state[self.cur][ypi + x]) +
                            u32::from(self.state[self.cur][ypi + xn]) +
                            u32::from(self.state[self.cur][yi + xp]) +
                            u32::from(self.state[self.cur][yi + xn]) +
                            u32::from(self.state[self.cur][yni + xp]) +
                            u32::from(self.state[self.cur][yni + x]) +
                            u32::from(self.state[self.cur][yni + xn]);

                self.state[1-self.cur][yi + x] = match (self.state[self.cur][yi + x], count) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (true, x) if x < 2 => false,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (true, 2) | (true, 3) => true,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (true, x) if x > 3 => false,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (false, 3) => true,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };
            }
        }
        self.cur = 1-self.cur;
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

/** How to show a block */
pub static BLOCK_SPRITE: [[u32; 4];8] = [
    [0x38c738c7, 0x38c738c7, 0x38c738c7, 0x38c738c7],
    [0x38c7718e, 0x718e718e, 0x718e718e, 0x718e38c7],
    [0x38c7718e, 0xaa55aa55, 0xaa55aa55, 0x718e38c7],
    [0x38c7718e, 0xaa55e31c, 0xe31caa55, 0x718e38c7],
    [0x38c7718e, 0xaa55e31c, 0xe31caa55, 0x718e38c7],
    [0x38c7718e, 0xaa55aa55, 0xaa55aa55, 0x718e38c7],
    [0x38c7718e, 0x718e718e, 0x718e718e, 0x718e38c7],
    [0x38c738c7, 0x38c738c7, 0x38c738c7, 0x38c738c7],
];

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

    let mut image: ScreenImage = [0; DISP_PIXELS / 2];

    writeln!(stdout, "NS2009 init").unwrap();
    let i2c = p.I2C0.constrain();
    i2c.init(NS2009_SLV_ADDR, NS2009_ADDR_BITS, NS2009_CLK);

    let mut filter = if let Some(filter) = TouchScreen::init(i2c, NS2009_CAL) {
        filter
    } else {
        writeln!(stdout, "NS2009 init failure").unwrap();
        panic!("Fatal error");
    };
    let mut universe = Universe::new();
    /* glider:
     010
     001
     111
     */
    universe.set(GRID_WIDTH/2+0, GRID_HEIGHT/2-1, true);
    universe.set(GRID_WIDTH/2+1, GRID_HEIGHT/2+0, true);
    universe.set(GRID_WIDTH/2-1, GRID_HEIGHT/2+1, true);
    universe.set(GRID_WIDTH/2+0, GRID_HEIGHT/2+1, true);
    universe.set(GRID_WIDTH/2+1, GRID_HEIGHT/2+1, true);
    loop {
        if let Some(ev) = filter.poll() {
            //writeln!(stdout, "{:?}", ev).unwrap();
            let x = ev.x / (BLK_SIZE as i32);
            let y = ev.y / (BLK_SIZE as i32);
            // Determine radius of changed area from pressure
            let r = (ev.z / 300) + 1;
            for yi in y-r..y+r+1 {
                for xi in x-r..x+r+1 {
                    let (xu, yu) = (xi as usize, yi as usize);
                    if xu < GRID_WIDTH && yu < GRID_HEIGHT {
                        universe.toggle(xu, yu);
                    }
                }
            }
        }
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let state = universe.get(x, y);
                for yi in 0..BLK_SIZE {
                    for xi in 0..BLK_SIZE/2 {
                        let idx = (y * BLK_SIZE + yi) * usize::from(DISP_WIDTH)/2 + x * BLK_SIZE/2 + xi;
                        image[idx] = if state { BLOCK_SPRITE[yi][xi] } else { 0 };
                    }
                }
            }
        }
        lcd.draw_picture(0, 0, DISP_WIDTH, DISP_HEIGHT, &image);

        universe.iterate();
    }
}
