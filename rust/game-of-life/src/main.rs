#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![no_std]
#![no_main]

extern crate panic_halt;

use k210_hal::pac;
use k210_hal::prelude::*;
use k210_hal::stdout::Stdout;
use k210_shared::board::def::{DISP_WIDTH,DISP_HEIGHT,NS2009_SLV_ADDR,NS2009_CAL,NS2009_ADDR_BITS,NS2009_CLK};
use k210_shared::board::lcd;
use k210_shared::board::lcd_colors;
use k210_shared::board::ns2009::TouchScreen;
use k210_shared::soc::fpioa;
use k210_shared::soc::i2c;
use k210_shared::soc::sleep::usleep;
use k210_shared::soc::sysctl;
use riscv_rt::entry;

pub const BLK_SIZE: usize = 8;
pub const GRID_WIDTH: usize = DISP_WIDTH / BLK_SIZE;
pub const GRID_HEIGHT: usize = DISP_HEIGHT / BLK_SIZE;

pub type ScreenImage = [u32; DISP_WIDTH * DISP_HEIGHT / 2];

struct Universe {
    state: [[bool; GRID_WIDTH*GRID_HEIGHT]; 2],
    cur: usize,
}

impl Universe {
    pub fn new() -> Self {
        Self {
            state: [[false; GRID_WIDTH*GRID_HEIGHT]; 2],
            cur: 0,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.state[self.cur][y * GRID_WIDTH + x]
    }

    pub fn set(&mut self, x: usize, y: usize, state: bool) {
        self.state[self.cur][y * GRID_WIDTH + x] = state;
    }

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

                let count = self.state[self.cur][ypi + xp] as u32 +
                            self.state[self.cur][ypi + x] as u32 +
                            self.state[self.cur][ypi + xn] as u32 +
                            self.state[self.cur][yi + xp] as u32 +
                            self.state[self.cur][yi + xn] as u32 +
                            self.state[self.cur][yni + xp] as u32 +
                            self.state[self.cur][yni + x] as u32 +
                            self.state[self.cur][yni + xn] as u32;

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
    fpioa::set_function(37, fpioa::function::gpiohs(lcd::RST_GPIONUM));
    fpioa::set_io_pull(37, fpioa::pull::DOWN); // outputs must be pull-down
    fpioa::set_function(38, fpioa::function::gpiohs(lcd::DCX_GPIONUM));
    fpioa::set_io_pull(38, fpioa::pull::DOWN);
    fpioa::set_function(36, fpioa::function::SPI0_SS3);
    fpioa::set_function(39, fpioa::function::SPI0_SCLK);

    /* I2C0 for touch-screen */
    fpioa::set_function(30, fpioa::function::I2C0_SCLK);
    fpioa::set_function(31, fpioa::function::I2C0_SDA);

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
    [0x28852885, 0x28852885, 0x28852885, 0x28852885],
    [0x2885512a, 0x512a512a, 0x512a512a, 0x512a2885],
    [0x2885512a, 0x79af79af, 0x79af79af, 0x512a2885],
    [0x2885512a, 0x79afaa55, 0xaa5579af, 0x512a2885],
    [0x2885512a, 0x79afaa55, 0xaa5579af, 0x512a2885],
    [0x2885512a, 0x79af79af, 0x79af79af, 0x512a2885],
    [0x2885512a, 0x512a512a, 0x512a512a, 0x512a2885],
    [0x28852885, 0x28852885, 0x28852885, 0x28852885],
];

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

    lcd::init();
    lcd::set_direction(lcd::direction::YX_LRUD);
    lcd::clear(lcd_colors::PURPLE);

    let mut image: ScreenImage = [0; DISP_WIDTH * DISP_HEIGHT / 2];

    writeln!(stdout, "NS2009 init").unwrap();
    i2c::init(NS2009_SLV_ADDR, NS2009_ADDR_BITS, NS2009_CLK);

    let mut filter = if let Some(filter) = TouchScreen::init(NS2009_CAL) {
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
            // writeln!(stdout, "{:?}", ev).unwrap();
            let x = ev.x / (BLK_SIZE as i32);
            let y = ev.y / (BLK_SIZE as i32);
            // Determine radius of changed area from pressure
            let r = ev.z / 300;
            for yi in y-r..y+r+1 {
                for xi in x-r..x+r+1 {
                    if (xi as usize) < DISP_WIDTH && (yi as usize) < DISP_HEIGHT {
                        universe.toggle(xi as usize, yi as usize);
                    }
                }
            }
        }
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let state = universe.get(x, y);
                for yi in 0..BLK_SIZE {
                    for xi in 0..BLK_SIZE/2 {
                        let idx = (y * BLK_SIZE + yi) * DISP_WIDTH/2 + x * BLK_SIZE/2 + xi;
                        image[idx] = if state { BLOCK_SPRITE[yi][xi] } else { 0 };
                    }
                }
            }
        }
        lcd::draw_picture(0, 0, DISP_WIDTH as u16, DISP_HEIGHT as u16, &image);

        universe.iterate();
    }
}
