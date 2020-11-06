//! Efficient(?) full-image rendering.
// TODO: switch this over to embedded-graphics probably
use crate::board::def::{DISP_HEIGHT, DISP_WIDTH, DISP_PIXELS};
use crate::board::lcd::LCDHL;

/** Array for representing an image of the entire screen.
 * This is an array of DISP_WIDTH / 2 × DISP_HEIGHT, each two horizontally consecutive
 * pixels are encoded in a u32 with `(b << 16)|a`.
 */
pub type ScreenImage = [u32; DISP_PIXELS / 2];

pub trait AsU8 {
    fn as_u8_slice(&self) -> &[u8];
    fn as_u8_slice_mut(&mut self) -> &mut [u8];
}

pub trait AsU16 {
    fn as_u16_slice(&self) -> &[u16];
    fn as_u16_slice_mut(&mut self) -> &mut [u16];
}

impl AsU8 for ScreenImage {
    fn as_u8_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.as_ptr() as *mut u8, DISP_PIXELS * 2) }
    }
    fn as_u8_slice_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.as_mut_ptr() as *mut u8, DISP_PIXELS * 2) }
    }
}

impl AsU16 for ScreenImage {
    fn as_u16_slice(&self) -> &[u16] {
        unsafe { core::slice::from_raw_parts(self.as_ptr() as *mut u16, DISP_PIXELS) }
    }
    fn as_u16_slice_mut(&mut self) -> &mut [u16] {
        unsafe { core::slice::from_raw_parts_mut(self.as_mut_ptr() as *mut u16, DISP_PIXELS) }
    }
}

pub fn render_image<L, I>(lcd: &mut L, mut image: I)
where
    L: LCDHL,
    I: FnMut(u16, u16) -> u16,
{
    // Theoretically this initialization could be avoided by directly initializing from an
    // iterator, however, rust doesn't have built-in functionality for this. There's a crate
    // (array_init) but it doesn't work for large arrays.
    let mut idata: ScreenImage = [0; DISP_PIXELS / 2];
    let yx = (0..DISP_HEIGHT)
        .flat_map(|y| core::iter::repeat(y).zip(0..DISP_WIDTH / 2));
    idata.iter_mut().zip(yx).for_each(|(v, (y, x))| {
        *v = (u32::from(image(x * 2 + 0, y)) << 0) | (u32::from(image(x * 2 + 1, y)) << 16);
    });

    // It would be possible to make draw_picture take an iterator directly
    // instead of rendering to an array first, however, this means that the
    // computation has to keep up with the SPI clock speed or there will be
    // glitches -- also it means that DMA cannot be used -- whereas a sufficiently
    // advanced DMA engine is indistinguishable from a GPU, the one in K210
    // isn't that.
    lcd.draw_picture(0, 0, DISP_WIDTH, DISP_HEIGHT, &idata);
}
