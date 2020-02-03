use k210_shared::board::lcd_colors::rgb565;

/** Basic color math. */
#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    #[allow(dead_code)]
    pub a: u8,
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
