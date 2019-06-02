//! Miscelleneous utilities for SoC functions (private)

pub fn set_bit(inval: u32, bit: u8, state: bool) -> u32 {
    if state {
        inval | (1 << (bit as u32))
    } else {
        inval & !(1 << (bit as u32))
    }
}

pub fn get_bit(inval: u32, bit: u8) -> bool {
    (inval & (1 << (bit as u32))) != 0
}
