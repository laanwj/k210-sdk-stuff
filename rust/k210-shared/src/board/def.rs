/** Global board definitions */

/** Display width in pixels */
pub const DISP_WIDTH: usize = 320;
/** Display height in pixels */
pub const DISP_HEIGHT: usize = 240;

/** I2C address of NS2009 */
pub const NS2009_SLV_ADDR: u16 = 0x48;

/** I2C address bits for NS2009 */
pub const NS2009_ADDR_BITS: u32 = 7;

/** I2C clock speed for NS2009 */
pub const NS2009_CLK: u32 = 100000;

/** Calibration matrix for touch screen */
pub const NS2009_CAL: [i32; 7] = [65, 5853, -1083592, -4292, -15, 16450115, 65536];
