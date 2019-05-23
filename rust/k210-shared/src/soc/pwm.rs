use k210_hal::pac;
// TODO: generalize over other timers than TIMER0
// use pac::{timer0,TIMER0,TIMER1,TIMER2};

use crate::soc::sysctl;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Channel {
    CH1 = 0,
    CH2,
    CH3,
    CH4,
}

/** Start a PWM channel */
pub fn pwm_start(ch: Channel) {
    unsafe {
        let ptr = pac::TIMER0::ptr();
        use pac::timer0::channel::control::MODEW;

        // set a deterministic value for load counts
        (*ptr).channel[ch as usize].load_count.write(|w| w.bits(1));
        (*ptr).load_count2[ch as usize].write(|w| w.bits(1));
        // start channel
        (*ptr).channel[ch as usize].control.write(
            |w| w.interrupt().set_bit()
                 .pwm_enable().set_bit()
                 .mode().variant(MODEW::USER)
                 .enable().set_bit());
    }
}

/** Stop a PWM channel */
pub fn pwm_stop(ch: Channel) {
    unsafe {
        let ptr = pac::TIMER0::ptr();

        (*ptr).channel[ch as usize].control.write(
            |w| w.interrupt().set_bit());
    }
}

/** Set frequency and value for a PWM channel */
pub fn pwm_set(ch: Channel, freq: u32, value: f32) -> u32 {
    let clk_freq = sysctl::clock_get_freq(sysctl::clock::TIMER0);
    let periods = clk_freq / freq;
    let percent = (value * (periods as f32)) as u32;
    unsafe {
        let ptr = pac::TIMER0::ptr();
        (*ptr).channel[ch as usize].load_count.write(|w| w.bits(periods - percent));
        (*ptr).load_count2[ch as usize].write(|w| w.bits(percent));
    }
    clk_freq / periods
}


