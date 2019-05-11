/** 32-bit LFSR for "random" output */
pub struct LFSR {
    state: u32,
    feedback: u32,
}

impl LFSR {
    pub fn new() -> LFSR {
        LFSR {
            state: 0x12345678,
            feedback: 0xf00f00f0, // LFSR period 0xf7ffffe0
        }
    }

    pub fn next(&mut self) -> u32 {
        let rv = self.state;
        let lsb = (self.state & 1) != 0;
        self.state >>= 1;
        if lsb {
            self.state ^= self.feedback;
        }
        rv
    }
}
