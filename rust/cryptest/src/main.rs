#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![no_std]
#![no_main]

use k210_hal::{Peripherals, pac};
use k210_hal::prelude::*;
use k210_hal::stdout::Stdout;
use k210_shared::soc::sysctl;
use k210_shared::soc::sleep::usleep;
use riscv::asm;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();

    let clocks = k210_hal::clock::Clocks::new();

    // Enable clocks for AES and reset the engine
    sysctl::clock_enable(sysctl::clock::AES);
    sysctl::reset(sysctl::reset::AES);

    // Configure UART
    let serial = p
        .UARTHS
        .configure((p.pins.pin5, p.pins.pin4), 115_200.bps(), &clocks);
    let (mut tx, _) = serial.split();
    let mut stdout = Stdout(&mut tx);

    usleep(200000);
    writeln!(
        stdout,
        "Init",
    ).unwrap();

    let aes = unsafe { &*pac::AES::ptr() };
    let sha256 = unsafe { &*pac::SHA256::ptr() };

    write!(stdout, "AES128: ").unwrap();
    let key: [u32; 4] = [0x16157e2b, 0xa6d2ae28, 0x8815f7ab, 0x3c4fcf09];
    let pt: [u32; 4] = [0xe2bec16b, 0x969f402e, 0x117e3de9, 0x2a179373];
    let ctref: [u32; 4] = [0xb47bd73a, 0x60367a0d, 0xf3ca9ea8, 0x97ef6624];
    let mut ct = [0u32; 4];
    unsafe {
        use pac::aes::mode_ctl::{CIPHER_MODE_A, KEY_MODE_A};
        use pac::aes::encrypt_sel::ENCRYPT_SEL_A;
        use pac::aes::en::EN_A;
        use pac::aes::endian::ENDIAN_A;
        use pac::aes::data_in_flag::DATA_IN_FLAG_A;
        use pac::aes::data_out_flag::DATA_OUT_FLAG_A;

        aes.endian.write(|w| w.endian().variant(ENDIAN_A::LE));

        for i in 0..4 {
            aes.key[i].write(|w| w.bits(key[3 - i]));
        }
        aes.mode_ctl.write(|w|
            w.cipher_mode().variant(CIPHER_MODE_A::ECB)
             .key_mode().variant(KEY_MODE_A::AES128));
        aes.encrypt_sel.write(|w|
            w.encrypt_sel().variant(ENCRYPT_SEL_A::ENCRYPTION));
        aes.aad_num.write(|w| w.bits(0));
        aes.pc_num.write(|w| w.bits(15));
        aes.en.write(|w|
            w.en().variant(EN_A::ENABLE));

        // Can write up to 80 bytes (20 words) here at once before the queue is full.
        for &v in pt.iter() {
            while aes.data_in_flag.read().data_in_flag() != DATA_IN_FLAG_A::CAN_INPUT {
                continue;
            }
            aes.text_data.write(|w| w.bits(v));
        }

        for i in 0..4 {
            while aes.data_out_flag.read().data_out_flag() != DATA_OUT_FLAG_A::CAN_OUTPUT {
                continue;
            }
            ct[i] = aes.out_data.read().bits();
            //write!(stdout, "{:08x} ", val).unwrap();
        }
        if ct == ctref {
            writeln!(stdout, "MATCH").unwrap();
        } else {
            writeln!(stdout, "MISMATCH").unwrap();
        }
    }
    loop {
        unsafe { asm::wfi(); }
    }
}
