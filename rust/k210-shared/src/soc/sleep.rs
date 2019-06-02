//! Utilities for sleeping short timespans
use crate::soc::sysctl;
use riscv::register::mcycle;

pub fn cycle_sleep(n: usize) {
    let start = mcycle::read();
    while (mcycle::read().wrapping_sub(start)) < n {
        // IDLE
    }
}

pub fn usleep(n: usize) {
    let freq = sysctl::clock_get_freq(sysctl::clock::CPU) as usize;
    cycle_sleep(freq * n / 1000000);
}
