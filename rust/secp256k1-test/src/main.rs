// secp256k1 in rust: testing
// https://github.com/rust-bitcoin/rust-secp256k1/blob/master/src/lib.rs
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![no_std]
#![no_main]

use k210_hal::pac::Peripherals;
use k210_hal::prelude::*;
use k210_hal::stdout::Stdout;
use riscv_rt::entry;
use k210_shared::soc::sleep::usleep;
use k210_shared::soc::sysctl;
use secp256k1::{Secp256k1, Message, SecretKey, PublicKey};
use core::slice;

/** Static buffer for secp256k1: make sure that this is 8-aligned
 * by reserving it as an array of u64.
 */
static mut SECP256K1_BUF: [u64; 70000/8] = [0u64; 70000/8];

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    sysctl::pll_set_freq(sysctl::pll::PLL0, 800_000_000).unwrap();
    sysctl::pll_set_freq(sysctl::pll::PLL1, 300_000_000).unwrap();
    sysctl::pll_set_freq(sysctl::pll::PLL2, 45_158_400).unwrap();
    let clocks = k210_hal::clock::Clocks::new();

    usleep(200000);

    let serial = p.UARTHS.configure(115_200.bps(), &clocks);
    let (mut tx, _) = serial.split();

    let mut stdout = Stdout(&mut tx);

    let bufsize = unsafe { core::mem::size_of_val(&SECP256K1_BUF) };
    writeln!(stdout, "testing {}(sign {} + verify {}) / {}",
             Secp256k1::preallocate_size(),
             Secp256k1::preallocate_signing_size(),
             Secp256k1::preallocate_verification_size(),
             bufsize).unwrap();
    assert!(Secp256k1::preallocate_size() < bufsize);

    let buf = unsafe { slice::from_raw_parts_mut(SECP256K1_BUF.as_mut_ptr() as *mut u8, bufsize) };
    writeln!(stdout, "secp initializing").unwrap();
    let secp = Secp256k1::preallocated_new(buf).unwrap();
    writeln!(stdout, "secp initialized {:?}", secp).unwrap();
    let secret_key = SecretKey::from_slice(&[0xcd; 32]).expect("32 bytes, within curve order");
    writeln!(stdout, "created secret key {:?}", secret_key).unwrap();
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    writeln!(stdout, "created public key {:?}", public_key).unwrap();
    let message = Message::from_slice(&[0xab; 32]).expect("32 bytes");
    writeln!(stdout, "created message {:?}", message).unwrap();

    let sig = secp.sign(&message, &secret_key);
    writeln!(stdout, "created signature {:?}", sig).unwrap();
    let result = secp.verify(&message, &sig, &public_key);
    writeln!(stdout, "verified signature {:?}", result).unwrap();

    loop {
    }
}
