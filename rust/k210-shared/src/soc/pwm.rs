//! TIMER peripherals (PWM handling)
use k210_hal::pac;
use core::ops::Deref;
use pac::{timer0,TIMER0,TIMER1,TIMER2};

use crate::soc::sysctl;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Channel {
    CH1 = 0,
    CH2,
    CH3,
    CH4,
}
impl Channel {
    pub fn idx(self) -> usize { self as usize }
}

pub trait TimerExt: Deref<Target = timer0::RegisterBlock> + Sized {
    #[doc(hidden)]
    const CLK: sysctl::clock;
    #[doc(hidden)]
    const DIV: sysctl::threshold;

    /// Constrains TIMER peripheral for PWM use
    /// A timer channel can either be used for PWM or as a normal timer (say, for interrupt
    /// generation). Currently this has a larger granularity than needed and
    /// constrains the entire peripheral for PWM use.
    fn constrain_pwm(self) -> PWMImpl<Self>;
}

impl TimerExt for TIMER0 {
    const CLK: sysctl::clock = sysctl::clock::TIMER0;
    const DIV: sysctl::threshold = sysctl::threshold::TIMER0;

    fn constrain_pwm(self) -> PWMImpl<Self> { PWMImpl::<Self> { timer: self } }
}
impl TimerExt for TIMER1 {
    const CLK: sysctl::clock = sysctl::clock::TIMER1;
    const DIV: sysctl::threshold = sysctl::threshold::TIMER1;

    fn constrain_pwm(self) -> PWMImpl<Self> { PWMImpl::<Self> { timer: self } }
}
impl TimerExt for TIMER2 {
    const CLK: sysctl::clock = sysctl::clock::TIMER2;
    const DIV: sysctl::threshold = sysctl::threshold::TIMER2;

    fn constrain_pwm(self) -> PWMImpl<Self> { PWMImpl::<Self> { timer: self } }
}

/** Trait for PWM control */
pub trait PWM {
    // TODO: make this per channel, and use the PWM trait from Rust embedded
    fn start(&self, ch: Channel);
    fn stop(&self, ch: Channel);
    fn set(&self, ch: Channel, freq: u32, value: f32) -> u32;
}

pub struct PWMImpl<TIMER> {
    timer: TIMER,
}

impl<TIMER: TimerExt> PWM for PWMImpl<TIMER> {
    /** Start a PWM channel */
    fn start(&self, ch: Channel) {
        unsafe {
            use pac::timer0::channel::control::MODE_A;

            // set a deterministic value for load counts
            self.timer.channel[ch.idx()].load_count.write(|w| w.bits(1));
            self.timer.load_count2[ch.idx()].write(|w| w.bits(1));
            // start channel
            self.timer.channel[ch.idx()].control.write(
                |w| w.interrupt().set_bit()
                     .pwm_enable().set_bit()
                     .mode().variant(MODE_A::USER)
                     .enable().set_bit());
        }
    }

    /** Stop a PWM channel */
    fn stop(&self, ch: Channel) {
        self.timer.channel[ch.idx()].control.write(
            |w| w.interrupt().set_bit());
    }

    /** Set frequency and value for a PWM channel */
    fn set(&self, ch: Channel, freq: u32, value: f32) -> u32 {
        let clk_freq = sysctl::clock_get_freq(TIMER::CLK);
        let periods = clk_freq / freq;
        let percent = (value * (periods as f32)) as u32;
        unsafe {
            self.timer.channel[ch.idx()].load_count.write(|w| w.bits(periods - percent));
            self.timer.load_count2[ch.idx()].write(|w| w.bits(percent));
        }
        clk_freq / periods
    }
}

