use core::fmt;

use k210_shared::board::lcd_colors::rgb565;
use crate::cp437;
use crate::palette_xterm256::PALETTE;

pub use k210_shared::board::def::{DISP_WIDTH,DISP_HEIGHT,DISP_PIXELS};
const GRID_WIDTH: u16 = DISP_WIDTH / 8;
const GRID_HEIGHT: u16 = DISP_HEIGHT / 8;
const GRID_CELLS: usize = (GRID_WIDTH as usize) * (GRID_HEIGHT as usize);
const DEF_FG: u16 = rgb565(192, 192, 192);
const DEF_BG: u16 = rgb565(0, 0, 0);

pub type ScreenImage = [u32; DISP_PIXELS / 2];

/* TODO
 * - pass in unicode to font mapping instead of hardcoding cp437
 */

/** Basic color math. */
#[derive(Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    #[allow(dead_code)]
    a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }

    pub const fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a: a }
    }

    pub const fn from_rgb565(val: u16) -> Color {
        let rs = ((val >> 11) & 0x1f) as u8;
        let gs = ((val >> 5) & 0x3f) as u8;
        let bs = ((val >> 0) & 0x1f) as u8;
        Color {
            r: (rs << 3) | (rs >> 2),
            g: (gs << 2) | (gs >> 4),
            b: (bs << 3) | (bs >> 2),
            a: 255,
        }
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

/** Integer screen coordinate. */
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

/** Cell flags. */
#[allow(non_snake_case)]
pub mod CellFlags {
    /** Cell contains a color font character. */
    pub const COLOR: u16 = 1;
}

/** One character cell */
#[derive(Copy, Clone)]
pub struct Cell {
    /** Background color in RGB565 */
    fg: u16,
    /** Background color in RGB565 */
    bg: u16,
    /** Font index. The only hard requirement on the font is that 0 is an empty glyph. */
    ch: u16,
    /** Cell flags (see CellFlags) */
    flags: u16,
}

enum State {
    Initial,
    Escape,
    CSI,
    Xterm,
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
    /** Standard font */
    pub font: &'static [[u8; 8]],
    /** Color font */
    pub color_font: Option<&'static [[u32; 32]]>,
    /** Dirty flag */
    pub dirty: bool,
    /** Array of character cells representing console */
    cells: [Cell; GRID_CELLS],
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
    pub fn new(font: &'static [[u8; 8]], color_font: Option<&'static [[u32; 32]]>) -> Console {
        Console {
            font, color_font,
            dirty: false,
            cells: [Cell {
                fg: DEF_FG,
                bg: DEF_BG,
                ch: 0,
                flags: 0,
            }; GRID_CELLS],
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
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH  {
                let cell = &self.cells[cell_idx];
                if (cell.flags & CellFlags::COLOR) != 0 {
                    if let Some(font) = self.color_font {
                        // glyph is a sequence of 32 (8*4) u32s, encoding two horizontal
                        // pixels each.
                        // TODO: do we want to highlight color font tiles when they're on the
                        // cursor?
                        let glyph = &font[usize::from(cell.ch)];
                        let mut image_ofs = image_base;
                        for yi in 0..8 {
                            for xih in 0..4 {
                                image[image_ofs + xih] = glyph[yi * 4 + xih];
                            }
                            image_ofs += usize::from(DISP_WIDTH) / 2;
                        }
                    }
                } else {
                    let glyph = &self.font[usize::from(cell.ch)];
                    let mut image_ofs = image_base;
                    let is_cursor =
                        self.cursor_visible && (y == self.cursor_pos.y) && (x == self.cursor_pos.x);
                    let fg = if is_cursor { cell.bg } else { cell.fg };
                    let bg = if is_cursor { cell.fg } else { cell.bg };
                    for yi in 0..8 {
                        let val = glyph[yi];
                        for xih in 0..4 {
                            image[image_ofs + xih] = (u32::from(if val & (1 << (xih * 2 + 0)) != 0 {
                                fg
                            } else {
                                bg
                            })
                                << 16)
                                | (u32::from(if val & (1 << (xih * 2 + 1)) != 0 {
                                    fg
                                } else {
                                    bg
                                })
                                    << 0);
                        }
                        image_ofs += usize::from(DISP_WIDTH) / 2;
                    }
                }
                cell_idx += 1;
                image_base += 8 / 2;
            }
            image_base += 7 * usize::from(DISP_WIDTH) / 2;
        }
    }

    pub fn width(&self) -> u16 {
        GRID_WIDTH
    }
    pub fn height(&self) -> u16 {
        GRID_HEIGHT
    }

    /** Put a char at an arbitrary position with arbitrary fg/bg color. Does not move the cursor.
     * Use this to regard the console as a simple grid of cells a la libtcod. Useful for drawing
     * frames and such.
     */
    pub fn put(&mut self, x: u16, y: u16, fg: Color, bg: Color, ch: char) {
        self.dirty = true;
        self.cells[usize::from(y) * usize::from(GRID_WIDTH) + usize::from(x)] = Cell {
            fg: rgb565(fg.r, fg.g, fg.b),
            bg: rgb565(bg.r, bg.g, bg.b),
            ch: u16::from(cp437::to(ch)),
            flags: 0,
        };
    }

    /** Raw put */
    pub fn put_raw(&mut self, x: u16, y: u16, fg: u16, bg: u16, ch: u16, flags: u16) {
        self.dirty = true;
        self.cells[usize::from(y) * usize::from(GRID_WIDTH) + usize::from(x)] = Cell {
            fg, bg, ch, flags
        };
    }

    /** Handle SGR escape sequence parameters */
    fn handle_sgr(&mut self) {
        let mut state = Sgr::Initial;
        let mut color = Color::new(0, 0, 0);
        for param in &self.num[0..self.idx+1] {
            match state {
                Sgr::Initial => {
                    match param {
                        0 => { self.cur_fg = self.def_fg; self.cur_bg = self.def_bg; }
                        30..=37 => { self.cur_fg = PALETTE[usize::from(param - 30)]; }
                        38 => { state = Sgr::SpecialFg; }
                        39 => { self.cur_fg = self.def_fg; }
                        40..=47 => { self.cur_bg = PALETTE[usize::from(param - 40)]; }
                        48 => { state = Sgr::SpecialBg; }
                        49 => { self.cur_bg = self.def_bg; }
                        90..=97 => { self.cur_fg = PALETTE[usize::from(8 + param - 90)]; }
                        100..=107 => { self.cur_bg = PALETTE[usize::from(8 + param - 100)]; }
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
                    self.cur_fg = PALETTE[usize::from(param & 0xff)];
                    state = Sgr::Initial;
                }
                Sgr::Bg256 => {
                    self.cur_bg = PALETTE[usize::from(param & 0xff)];
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

    /** Scroll (only up, currently) */
    pub fn scroll(&mut self) {
        let gw = usize::from(GRID_WIDTH);
        let gh = usize::from(GRID_HEIGHT);
        for i in 0..(gh-1)*gw {
            self.cells[i] = self.cells[i + gw];
        }
        for i in 0..GRID_WIDTH {
            self.cells[(gh-1)*gw + usize::from(i)] = Cell {
                fg: self.cur_fg,
                bg: self.cur_bg,
                ch: 0,
                flags: 0,
            };
        }
        if self.cursor_pos.y > 0 {
            self.cursor_pos.y -= 1;
        }
        self.dirty = true;
    }

    /** Put a char at current cursor position, interpreting control and escape codes. */
    pub fn putch(&mut self, ch: char) {
        match self.state {
            State::Initial => match ch {
                '\x08' => { // backspace
                    if self.cursor_pos.x > 0 {
                        self.cursor_pos.x -= 1;
                        self.put_raw(self.cursor_pos.x, self.cursor_pos.y, self.cur_fg, self.cur_bg, 0, 0);
                    }
                }
                '\r' => { self.cursor_pos.x = 0; self.dirty = true; }
                '\n' => {
                    self.cursor_pos.y += 1; self.cursor_pos.x = 0; self.dirty = true;
                    if self.cursor_pos.y == GRID_HEIGHT {
                        self.scroll();
                    }
                }
                '\x1b' => { self.state = State::Escape; }
                '\x00'..='\x1f' => {
                    // Unhandled control character, skip it
                }
                ch => {
                    // allow cursor to be at 'virtual' column GRID_WIDTH to allow using all
                    // (limited number of) columns
                    if self.cursor_pos.x == GRID_WIDTH {
                        self.cursor_pos.x = 0;
                        self.cursor_pos.y += 1;
                    }
                    if self.cursor_pos.y == GRID_HEIGHT {
                        self.scroll();
                    }

                    self.put_raw(self.cursor_pos.x, self.cursor_pos.y, self.cur_fg, self.cur_bg, cp437::to(ch).into(), 0);
                    self.cursor_pos.x += 1;
                }
            }
            State::Escape => match ch {
                '[' => { self.state = State::CSI; self.idx = 0; self.num[0] = 0; }
                ']' => { self.state = State::Xterm; }
                _ => { self.state = State::Initial; }
            }
            State::CSI => match ch {
                '0'..='9' => {
                    self.num[self.idx] = self.num[self.idx].wrapping_mul(10).wrapping_add(((ch as u8) - b'0').into());
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
            // This sets window title and such, we can't do anything with this information so
            // ignore until the BEL
            State::Xterm => match ch {
                    '\x07' => {
                        self.state = State::Initial;
                    }
                    _ => { }
            }
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
