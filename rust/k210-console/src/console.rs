use core::fmt;

use k210_shared::board::lcd_colors::rgb565;
use crate::cp437;
use crate::cp437_8x8;
use crate::palette_xterm256::PALETTE;

pub const DISP_WIDTH: usize = 320;
pub const DISP_HEIGHT: usize = 240;
const GRID_WIDTH: usize = DISP_WIDTH / 8;
const GRID_HEIGHT: usize = DISP_HEIGHT / 8;
const DEF_FG: u16 = rgb565(192, 192, 192);
const DEF_BG: u16 = rgb565(0, 0, 0);

pub type ScreenImage = [u32; DISP_WIDTH * DISP_HEIGHT / 2];

#[derive(Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }

    pub const fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a: a }
    }

    pub const fn from_rgba32(val: u32) -> Color {
        Color {
            r: ((val >> 24) & 0xff) as u8,
            g: ((val >> 16) & 0xff) as u8,
            b: ((val >> 8) & 0xff) as u8,
            a: ((val >> 0) & 0xff) as u8,
        }
    }

    pub const fn to_rgb565(&self) -> u16 {
        rgb565(self.r, self.g, self.b)
    }
}

#[derive(Copy, Clone)]
pub struct Coord {
    x: u16,
    y: u16,
}

impl Coord {
    fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

/** Axis aligned 2D rectangle */
#[derive(Copy, Clone)]
pub struct Rect {
    x0: u16,
    y0: u16,
    x1: u16,
    y1: u16,
}

impl Rect {
    fn new(x0: u16, y0: u16, x1: u16, y1: u16) -> Self {
        Self { x0, y0, x1, y1 }
    }
}

/** One character cell */
#[derive(Copy, Clone)]
pub struct Cell {
    fg: u16,
    bg: u16,
    ch: char,
}

enum State {
    Initial,
    Escape,
    CSI,
}

enum Sgr {
    Initial,
    SpecialFg,
    SpecialBg,
    Fg256,
    Bg256,
    FgR,
    BgR,
    FgG,
    BgG,
    FgB,
    BgB,
}

/** Visual attributes of console */
pub struct Console {
    /** Dirty flag */
    pub dirty: bool,
    /** Array of character cells representing console */
    cells: [Cell; GRID_WIDTH * GRID_HEIGHT],
    /** Cursor position */
    cursor_pos: Coord,
    /** Cursor visible flag */
    cursor_visible: bool,
    /** Default foreground */
    def_fg: u16,
    /** Default background */
    def_bg: u16,
    /** Current foreground */
    cur_fg: u16,
    /** Current background */
    cur_bg: u16,
    /** Current escape state */
    state: State,
    /** Current CSI parameter */
    idx: usize,
    /** CSI parameters */
    num: [u16; 16],
}

impl Console {
    /** Create new, empty console */
    pub fn new() -> Console {
        Console {
            dirty: false,
            cells: [Cell {
                fg: DEF_FG,
                bg: DEF_BG,
                ch: '\x00',
            }; GRID_WIDTH * GRID_HEIGHT],
            cursor_pos: Coord::new(0, 0),
            cursor_visible: true,
            def_fg: DEF_FG,
            def_bg: DEF_BG,
            cur_fg: DEF_FG,
            cur_bg: DEF_BG,
            state: State::Initial,
            idx: 0,
            num: [0; 16],
        }
    }

    /** Render console to u32 image for ST7789V LCD */
    pub fn render(&self, image: &mut ScreenImage) {
        let mut image_base = 0;
        let mut cell_idx = 0;
        for y in 0..(GRID_HEIGHT as u16) {
            for x in 0..(GRID_WIDTH as u16) {
                let cell = &self.cells[cell_idx];
                let glyph = &cp437_8x8::FONT[cp437::to(cell.ch) as usize];
                let mut image_ofs = image_base;
                let is_cursor =
                    self.cursor_visible && (y == self.cursor_pos.y) && (x == self.cursor_pos.x);
                let fg = if is_cursor { cell.bg } else { cell.fg };
                let bg = if is_cursor { cell.fg } else { cell.bg };
                for yi in 0..8 {
                    let val = glyph[yi];
                    for xih in 0..4 {
                        image[image_ofs + xih] = ((if val & (1 << (xih * 2 + 0)) != 0 {
                            fg
                        } else {
                            bg
                        } as u32)
                            << 16)
                            | ((if val & (1 << (xih * 2 + 1)) != 0 {
                                fg
                            } else {
                                bg
                            } as u32)
                                << 0);
                    }
                    image_ofs += DISP_WIDTH / 2;
                }
                cell_idx += 1;
                image_base += 8 / 2;
            }
            image_base += 7 * DISP_WIDTH / 2;
        }
    }

    pub fn width(&self) -> u16 {
        GRID_WIDTH as u16
    }
    pub fn height(&self) -> u16 {
        GRID_HEIGHT as u16
    }

    /** Put a char at an arbitrary position with arbitrary fg/bg color. Does not move the cursor.
     * Use this to regard the console as a simple grid of cells a la libtcod. Useful for drawing
     * frames and such.
     */
    pub fn put(&mut self, x: u16, y: u16, fg: Color, bg: Color, ch: char) {
        self.dirty = true;
        self.cells[(y as usize) * GRID_WIDTH + (x as usize)] = Cell {
            fg: rgb565(fg.r, fg.g, fg.b),
            bg: rgb565(bg.r, bg.g, bg.b),
            ch,
        };
    }

    /** Handle SGR escape sequence parameters */
    pub fn handle_sgr(&mut self) {
        let mut state = Sgr::Initial;
        let mut color = Color::new(0, 0, 0);
        for param in &self.num[0..self.idx+1] {
            match state {
                Sgr::Initial => {
                    match param {
                        0 => { self.cur_fg = self.def_fg; self.cur_bg = self.def_bg; }
                        30..=37 => { self.cur_fg = Color::from_rgba32(PALETTE[(param - 30) as usize]).to_rgb565(); }
                        38 => { state = Sgr::SpecialFg; }
                        40..=47 => { self.cur_bg = Color::from_rgba32(PALETTE[(param - 40) as usize]).to_rgb565(); }
                        48 => { state = Sgr::SpecialBg; }
                        90..=97 => { self.cur_fg = Color::from_rgba32(PALETTE[8 + (param - 90) as usize]).to_rgb565(); }
                        100..=107 => { self.cur_bg = Color::from_rgba32(PALETTE[8 + (param - 100) as usize]).to_rgb565(); }
                        _ => {}
                    }
                }
                Sgr::SpecialFg => {
                    match param {
                        2 => { state = Sgr::FgR; }
                        5 => { state = Sgr::Fg256; }
                        _ => { state = Sgr::Initial; }
                    }
                }
                Sgr::SpecialBg => {
                    match param {
                        2 => { state = Sgr::BgR; }
                        5 => { state = Sgr::Bg256; }
                        _ => { state = Sgr::Initial; }
                    }
                }
                Sgr::Fg256 => {
                    self.cur_fg = Color::from_rgba32(PALETTE[(param & 0xff) as usize]).to_rgb565();
                    state = Sgr::Initial;
                }
                Sgr::Bg256 => {
                    self.cur_bg = Color::from_rgba32(PALETTE[(param & 0xff) as usize]).to_rgb565();
                    state = Sgr::Initial;
                }
                Sgr::FgR => { color.r = (param & 0xff) as u8; state = Sgr::FgG; }
                Sgr::FgG => { color.g = (param & 0xff) as u8; state = Sgr::FgB; }
                Sgr::FgB => { color.b = (param & 0xff) as u8; state = Sgr::Initial; self.cur_fg = color.to_rgb565(); }
                Sgr::BgR => { color.r = (param & 0xff) as u8; state = Sgr::BgG; }
                Sgr::BgG => { color.g = (param & 0xff) as u8; state = Sgr::BgB; }
                Sgr::BgB => { color.b = (param & 0xff) as u8; state = Sgr::Initial; self.cur_bg = color.to_rgb565(); }
            }
        }
    }

    /** Put a char at current cursor position, interpreting control and escape codes. */
    pub fn putch(&mut self, ch: char) {
        match self.state {
            State::Initial => {
                match ch {
                    '\r' => { self.cursor_pos.x = 0; }
                    '\n' => { self.cursor_pos.y += 1; self.cursor_pos.x = 0; }
                    '\x1b' => { self.state = State::Escape; }
                    ch => {
                        self.dirty = true;
                        self.cells[(self.cursor_pos.y as usize) * GRID_WIDTH + (self.cursor_pos.x as usize)] = Cell {
                            fg: self.cur_fg,
                            bg: self.cur_bg,
                            ch,
                        };
                        self.cursor_pos.x += 1;
                    }
                }
            }
            State::Escape => {
                match ch {
                    '[' => { self.state = State::CSI; self.idx = 0; self.num[0] = 0; }
                    _ => { self.state = State::Initial; }
                }
            }
            State::CSI => {
                match ch {
                    '0'..='9' => {
                        self.num[self.idx] *= 10;
                        self.num[self.idx] += ((ch as u8) - b'0') as u16;
                    }
                    ';' => {
                        self.idx += 1;
                        if self.idx == self.num.len() {
                            // Too many arguments, ignore sequence
                            self.state = State::Initial;
                        } else {
                            self.num[self.idx] = 0;
                        }
                    }
                    'm' => {
                        self.handle_sgr();
                        self.state = State::Initial;
                    }
                    _ => {
                        self.state = State::Initial;
                    }
                }
            }
        }
        // wrap around
        if self.cursor_pos.x == GRID_WIDTH as u16 {
            self.cursor_pos.x = 0;
            self.cursor_pos.y += 1;
        }
        if self.cursor_pos.y == GRID_HEIGHT as u16 {
            self.cursor_pos.y = 0;
        }
    }

    /** Put a string at current cursor position, interpreting control and escape codes. */
    pub fn puts(&mut self, s: &str) {
        for ch in s.chars() {
            self.putch(ch);
        }
    }
}

/** Formatting adoption for console */
impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> { self.puts(s); Ok(()) }
    fn write_char(&mut self, c: char) -> Result<(), fmt::Error> { self.putch(c); Ok(()) }
}
