pub static WIDTH: u32 = 256;
pub static HEIGHT: u32 = 256;
pub static VOXEL_MAP: &'static [u8] = include_bytes_align_as!(u16, "../data/map15.dat");
