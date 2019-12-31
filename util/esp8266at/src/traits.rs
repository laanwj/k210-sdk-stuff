use core::fmt;
#[cfg(feature = "std")]
use std::io;

/** The trait that's required of anything acting as serial port writer.
 * It is much simpler than io::Write. The reason for implementing our own trait here
 * is that this is compatible with no_std.
 */
pub trait Write {
    type Error: fmt::Debug;

    /** Write all bytes from the buffer, or fail */
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error>;
}

/** Implement our write trait for everything that implements io::Write */
#[cfg(feature = "std")]
impl<X> Write for X
where
    X: io::Write,
{
    type Error = io::Error;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        (self as &mut dyn io::Write).write_all(buf)
    }
}
