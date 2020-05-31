/** Simple logging functionality that prints to UARTHS at the current settings.
 * The advantage of this is that it can be used from anywhere without having to pass
 * stdout around. The disadvantage is that this prevents rust's safety rules from preventing
 * concurrent access. It also assumes someone else set up the UART already.
 */
use k210_hal::pac;
pub use core::fmt::Write;

pub struct DebugLogger;

impl Write for DebugLogger {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let uart = unsafe {&*pac::UARTHS::ptr() };
        for &byte in s.as_bytes() {
            while uart.txdata.read().full().bit_is_set() {
                continue;
            }
            unsafe {
                uart.txdata.write(|w| w.data().bits(byte));
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! debugln {
    ($($arg:tt)+) => ({
        let mut stdout = $crate::debug::DebugLogger {};
        writeln!(stdout, $($arg)+).unwrap();
    })
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => ({
        let mut stdout = $crate::debug::DebugLogger {};
        write!(stdout, $($arg)+).unwrap();
    })
}
