/** Panic handler: based on ARM panic-itm */
use k210_hal::pac::Peripherals;
use k210_hal::prelude::*;
use k210_hal::stdout::Stdout;
use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

/** Send panic messages to UARTHS at 115200 baud */
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Stealing all peripherals, re-initializing the clocks and serial seems overkill here, but
    // also, can we really know the state?
    let p = unsafe { Peripherals::steal() };
    let clocks = k210_hal::clock::Clocks::new();
    let serial = p.UARTHS.configure(115_200.bps(), &clocks);
    let (mut tx, _) = serial.split();
    let mut stdout = Stdout(&mut tx);
    writeln!(stdout, "{}", info).unwrap();

    loop {
        // add some side effect to prevent this from turning into a UDF instruction
        // see rust-lang/rust#28728 for details
        atomic::compiler_fence(Ordering::SeqCst)
    }
}
