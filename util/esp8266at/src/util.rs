use core::slice;

use crate::traits::Write;

/** Write quoted string. `\` and `"` are escaped, and the string
 * is automatically surrounded with double-quotes.
 */
pub fn write_qstr<W>(w: &mut W, s: &[u8]) -> Result<(), W::Error>
where
    W: Write,
{
    w.write_all(b"\"")?;
    for ch in s {
        w.write_all(match ch {
            b'\"' => &[b'\\', b'"'],
            b'\\' => &[b'\\', b'\\'],
            _ => slice::from_ref(ch),
        })?;
    }
    w.write_all(b"\"")?;
    Ok(())
}

/** Write decimal unsigned number */
pub fn write_num_u32<W>(w: &mut W, mut val: u32) -> Result<(), W::Error>
where
    W: Write,
{
    let mut buf = [0u8; 10];
    let mut curr = buf.len();
    for byte in buf.iter_mut().rev() {
        *byte = b'0' + (val % 10) as u8;
        val = val / 10;
        curr -= 1;
        if val == 0 {
            break;
        }
    }
    w.write_all(&buf[curr..])
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrayvec::ArrayVec;

    #[cfg(not(feature = "std"))]
    use arrayvec::Array;
    #[cfg(not(feature = "std"))]
    impl<A: Array<Item = u8>> Write for ArrayVec<A> {
        type Error = ();
        fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
            for byte in buf {
                self.push(*byte); // Panics if the vector is already full.
            }
            Ok(())
        }
    }

    #[test]
    fn test_qstr() {
        let mut o = ArrayVec::<[_; 16]>::new();
        write_qstr(&mut o, b"123").unwrap();
        assert_eq!(o.as_slice(), b"\"123\"");

        o.clear();
        write_qstr(&mut o, b"\"\\").unwrap();
        assert_eq!(o.as_slice(), b"\"\\\"\\\\\"");
    }

    #[test]
    fn test_num() {
        let mut o = ArrayVec::<[_; 16]>::new();
        write_num_u32(&mut o, 123).unwrap();
        assert_eq!(o.as_slice(), b"123");

        o.clear();
        write_num_u32(&mut o, 0).unwrap();
        assert_eq!(o.as_slice(), b"0");

        o.clear();
        write_num_u32(&mut o, 4294967295).unwrap();
        assert_eq!(o.as_slice(), b"4294967295");
    }
}
