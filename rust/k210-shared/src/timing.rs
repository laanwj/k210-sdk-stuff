/** Utilities for measuring time and framerates. */
use core::cmp;
use crate::soc::sysctl;
use riscv::register::mcycle;

/** Number of frame times to store. */
const N_FRAMES: usize = 100;
/** Use last N seconds for statistics. */
const N_SEC: u64 = 2;

/** Counter for FPS statistics.
 */
pub struct FPSTimer {
    /** An array of the times of the last N frames. */
    frame_times: [u64; N_FRAMES],
    /** Current offset in ring buffer. */
    ofs: usize,
}

impl FPSTimer {
    pub fn new() -> Self {
        Self {
            frame_times: [0; N_FRAMES],
            ofs: 0,
        }
    }

    pub fn frame(&mut self) {
        self.frame_times[self.ofs] = mcycle::read64();
        self.ofs += 1;
        if self.ofs == N_FRAMES {
            self.ofs = 0;
        }
    }

    pub fn fps(&self) -> u32 {
        let freq = sysctl::clock_get_freq(sysctl::clock::CPU) as u64;
        let now = mcycle::read64();
        let oldest = now.saturating_sub(freq * N_SEC);
        let mut count: u32 = 0;
        let mut min: u64 = u64::max_value();
        for &t in self.frame_times.iter() {
            if t > oldest {
                count += 1;
            }
            if t != 0 {
                min = cmp::min(min, t);
            }
        }
        if count == 0 {
            0
        } else if min <= oldest {
            // Collected full N seconds
            count / (N_SEC as u32)
        } else {
            // Collected less than N seconds, make an estimate based on what we have
            (u64::from(count) * freq / (now - min)) as u32
        }
    }
}

/** Return time in microseconds. The starting point is undefined, only differences make sense. */
pub fn clock() -> u64 {
    let freq = sysctl::clock_get_freq(sysctl::clock::CPU) as u64;
    let cycles = mcycle::read64();
    return cycles * 1_000_000 / freq;
}
