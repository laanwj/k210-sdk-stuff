/** Example synchronous serial receive event loop (for std) */
use std::fmt;
use std::io;

use crate::handler::{NetworkEvent, SerialNetworkHandler};
use crate::response::{parse, ParseResult};

/** Mainloop handling serial input and dispatching network events */
pub fn mainloop<P, F, X>(
    h: &mut SerialNetworkHandler<X>,
    port: &mut P,
    mut f: F,
    debug: &mut dyn fmt::Write,
) -> io::Result<()>
where
    P: io::Read,
    F: FnMut(&mut SerialNetworkHandler<X>, NetworkEvent, &mut dyn fmt::Write) -> bool,
    X: io::Write,
{
    let mut serial_buf: Vec<u8> = vec![0; 2560]; // 2048 + some
    let mut ofs: usize = 0;
    let mut running: bool = true;
    while running {
        // Receive bytes into buffer
        match port.read(&mut serial_buf[ofs..]) {
            Ok(t) => {
                // io::stdout().write_all(&serial_buf[ofs..ofs+t]).unwrap();
                ofs += t;

                // Loop as long as there's something in the buffer to parse, starting at the
                // beginning
                let mut start = 0;
                while start < ofs {
                    // try parsing
                    let tail = &serial_buf[start..ofs];
                    let erase = match parse(tail) {
                        ParseResult::Ok(offset, resp) => {
                            h.message(
                                &resp,
                                |a, b, debug| {
                                    running = f(a, b, debug);
                                },
                                debug,
                            )?;

                            offset
                        }
                        ParseResult::Incomplete => {
                            // Incomplete, ignored, just retry after a new receive
                            0
                        }
                        ParseResult::Err => {
                            writeln!(debug, "err: {:?}", tail).unwrap();
                            // Erase unparseable data to next line, if line is complete
                            if let Some(ofs) = tail.iter().position(|&x| x == b'\n') {
                                ofs + 1
                            } else {
                                // If not, retry next time
                                0
                            }
                        }
                    };
                    if erase == 0 {
                        // End of input or remainder unparseable
                        break;
                    }
                    start += erase;
                }
                // Erase everything before new starting offset
                for i in start..ofs {
                    serial_buf[i - start] = serial_buf[i];
                }
                ofs -= start;
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => return Err(e),
        }
    }
    Ok(())
}
