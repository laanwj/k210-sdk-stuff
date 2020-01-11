/** Align an array of bytes to a specified alignment. This struct is generic in Bytes to admit unsizing coercions.
 * See: https://users.rust-lang.org/t/can-i-conveniently-compile-bytes-into-a-rust-program-with-a-specific-alignment/24049
 */
#[repr(C)] // guarantee 'bytes' comes after '_align'
struct AlignedTo<Align, Bytes: ?Sized> {
    _align: [Align; 0],
    bytes: Bytes,
}

/** Dummy static used to create aligned data. */
static ALIGNED: &'static AlignedTo<u16, [u8]> = &AlignedTo {
    _align: [],
    bytes: *include_bytes!("../data/map15.dat"),
};

pub static WIDTH: u32 = 256;
pub static HEIGHT: u32 = 256;
pub static VOXEL_MAP: &'static [u8] = &ALIGNED.bytes;
