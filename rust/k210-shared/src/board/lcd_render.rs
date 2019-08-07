//! Efficient(?) full-image rendering.
// TODO: switch this over to embedded-graphics probably
use crate::board::def::{DISP_HEIGHT, DISP_WIDTH};
use crate::board::lcd::LCDHL;

/** Array for representing an image of the entire screen.
 * This is an array of DISP_WIDTH / 2 × DISP_HEIGHT, each two horizontally consecutive
 * pixels are encoded in a u32 with `(a << 16)|b`.
 */
pub type ScreenImage = [u32; DISP_WIDTH * DISP_HEIGHT / 2];

pub fn render_image<L, I>(lcd: &mut L, mut image: I)
where
    L: LCDHL,
    I: FnMut(u16, u16) -> u16,
{
    // Theoretically this initialization could be avoided by directly initializing from an
    // iterator, however, rust doesn't have built-in functionality for this. There's a crate
    // (array_init) but it doesn't work for large arrays.
    let mut idata: ScreenImage = [0; DISP_WIDTH * DISP_HEIGHT / 2];
    let yx = (0..DISP_HEIGHT)
        .flat_map(|y| core::iter::repeat(y as u16).zip(0 as u16..(DISP_WIDTH / 2) as u16));
    idata.iter_mut().zip(yx).for_each(|(v, (y, x))| {
        *v = ((image(x * 2 + 0, y) as u32) << 16) | (image(x * 2 + 1, y) as u32);
    });

    // It would be possible to make draw_picture take an iterator directly
    // instead of rendering to an array first, however, this means that the
    // computation has to keep up with the SPI clock speed or there will be
    // glitches -- also it means that DMA cannot be used -- whereas a sufficiently
    // advanced DMA engine is indistinguishable from a GPU, the one in K210
    // isn't that.
    lcd.draw_picture(0, 0, DISP_WIDTH as u16, DISP_HEIGHT as u16, &idata);
}
