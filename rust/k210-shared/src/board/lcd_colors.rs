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
    return (((r as u16) >> 3) << 11) | (((g as u16) >> 2) << 5) | ((b as u16) >> 3);
}
