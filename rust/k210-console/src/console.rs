use crate::board::lcd_colors::rgb565;
use crate::cp437;
use crate::cp437_8x8;

pub const DISP_WIDTH: usize = 320;
pub const DISP_HEIGHT: usize = 240;
const GRID_WIDTH: usize = DISP_WIDTH / 8;
const GRID_HEIGHT: usize = DISP_HEIGHT / 8;

pub type ScreenImage = [u32; DISP_WIDTH * DISP_HEIGHT / 2];

#[derive(Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }

    pub fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a: a }
    }

    pub fn from_rgba32(val: u32) -> Color {
        Color {
            r: ((val >> 24) & 0xff) as u8,
            g: ((val >> 16) & 0xff) as u8,
            b: ((val >> 8) & 0xff) as u8,
            a: ((val >> 0) & 0xff) as u8,
        }
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

/** Visual attributes of console */
pub struct Console {
    /** Array of character cells representing console */
    cells: [Cell; GRID_WIDTH * GRID_HEIGHT],
    /** Cursor position */
    cursor_pos: Coord,
    /** Cursor visible flag */
    cursor_visible: bool,
}

impl Console {
    /** Create new, empty console */
    pub fn new() -> Console {
        Console {
            cells: [Cell {
                fg: 0,
                bg: 0,
                ch: '\x00',
            }; GRID_WIDTH * GRID_HEIGHT],
            cursor_pos: Coord::new(0, 0),
            cursor_visible: true,
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

    pub fn put(&mut self, x: u16, y: u16, fg: Color, bg: Color, ch: char) {
        self.cells[(y as usize) * GRID_WIDTH + (x as usize)] = Cell {
            fg: rgb565(fg.r, fg.g, fg.b),
            bg: rgb565(bg.r, bg.g, bg.b),
            ch,
        };
    }
}
