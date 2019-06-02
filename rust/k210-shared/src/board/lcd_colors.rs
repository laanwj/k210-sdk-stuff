//! Helper functions for LCD color handling
use libm::F32Ext;

/** Some convenient RGB565 colors */
pub const BLACK: u16 = 0x0000;
pub const NAVY: u16 = 0x000F;
pub const DARKGREEN: u16 = 0x03E0;
pub const DARKCYAN: u16 = 0x03EF;
pub const MAROON: u16 = 0x7800;
pub const PURPLE: u16 = 0x780F;
pub const OLIVE: u16 = 0x7BE0;
pub const LIGHTGREY: u16 = 0xC618;
pub const DARKGREY: u16 = 0x7BEF;
pub const BLUE: u16 = 0x001F;
pub const GREEN: u16 = 0x07E0;
pub const CYAN: u16 = 0x07FF;
pub const RED: u16 = 0xF800;
pub const MAGENTA: u16 = 0xF81F;
pub const YELLOW: u16 = 0xFFE0;
pub const WHITE: u16 = 0xFFFF;
pub const ORANGE: u16 = 0xFD20;
pub const GREENYELLOW: u16 = 0xAFE5;
pub const PINK: u16 = 0xF81F;

/** Truncate 8 bit RGB to RBG565 */
pub const fn rgb565(r: u8, g: u8, b: u8) -> u16 {
    (((r as u16) >> 3) << 11) | (((g as u16) >> 2) << 5) | ((b as u16) >> 3)
}

/** 32.0 minus 1ulp */
const ALMOST_32: f32 = 31.999998f32;
/** 64.0 minus 1ulp */
const ALMOST_64: f32 = 63.999996f32;

/** Truncate 32 bit RGB to RBG565 */
pub fn rgbf565(r: f32, g: f32, b: f32) -> u16 {
    (((r * ALMOST_32) as u16) << 11) |
      (((g * ALMOST_64) as u16) << 5) |
      ((b * ALMOST_32) as u16)
}

/** HSV to RGB. `h` is 0.0..360.0, `s` and `v` are 0.0..1.0 output RGB will be 0.0..1.0 (all ranges
 * inclusive)
 */
pub fn hsv2rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let h = h / 60.0;
    let i = h.trunc();
    let f = h - i;

    let c = v * (1.0 - s * f);
    let b = v * (1.0 - s + s * f);
    let o = v * (1.0 - s);
    match i as u32 {
        // yellow to green
        1 => (c, v, o),
        // green to cyan
        2 => (o, v, b),
        // cyan to blue
        3 => (o, c, v),
        // blue to magenta
        4 => (b, o, v),
        // magenta to red
        5 => (v, o, c),
        // red to yellow
        _ => (v, b, o),
    }
}

/** Clamp a float between 0 and 1 */
pub fn clampf(v: f32) -> f32 {
    if v < 0.0 {
        0.0
    } else if v > 1.0 {
        1.0
    } else {
        v
    }
}
