#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![no_std]
#![no_main]

use core::sync::atomic::{self, Ordering};
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
    // Enable clocks for SHA256 and reset the engine
    sysctl::clock_enable(sysctl::clock::SHA);
    sysctl::reset(sysctl::reset::SHA);

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
                atomic::compiler_fence(Ordering::SeqCst)
            }
            aes.text_data.write(|w| w.bits(v));
        }

        for i in 0..4 {
            while aes.data_out_flag.read().data_out_flag() != DATA_OUT_FLAG_A::CAN_OUTPUT {
                atomic::compiler_fence(Ordering::SeqCst)
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
    /*
    $ echo -n "abc" | sha256sum
    ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad  -
    */
    let sha_in: [u8; 64] = [
        b'a', b'b', b'c', 0x80, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, // Padding (total size in bits, big-endian)
    ];
    let sha_out_ref: [u8; 32] = [
        0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea,
        0x41, 0x41, 0x40, 0xde, 0x5d, 0xae, 0x22, 0x23,
        0xb0, 0x03, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c,
        0xb4, 0x10, 0xff, 0x61, 0xf2, 0x00, 0x15, 0xad,
    ];
    let mut sha_out = [0u8; 32];
    write!(stdout, "SHA256: ").unwrap();
    unsafe {
        use pac::sha256::function_reg_0::{ENDIAN_A};
        sha256.num_reg.write(|w|
            w.data_cnt().bits(1));
        sha256.function_reg_0.write(|w|
            w.endian().variant(ENDIAN_A::BE)
             .en().bit(true));
        sha256.function_reg_1.write(|w|
            w.dma_en().bit(false));
        for i in 0..16 {
            while sha256.function_reg_1.read().fifo_in_full().bit() {
                atomic::compiler_fence(Ordering::SeqCst)
            }
            sha256.data_in.write(|w| w.bits(
                    ((sha_in[i*4 + 0] as u32) << 0)
                  | ((sha_in[i*4 + 1] as u32) << 8)
                  | ((sha_in[i*4 + 2] as u32) << 16)
                  | ((sha_in[i*4 + 3] as u32) << 24)
                ));
        }
        while !sha256.function_reg_0.read().en().bit() {
            atomic::compiler_fence(Ordering::SeqCst)
        }
        for i in 0..8 {
            let val = sha256.result[7 - i].read().bits();
            sha_out[i*4 + 0] = (val >> 0) as u8;
            sha_out[i*4 + 1] = (val >> 8) as u8;
            sha_out[i*4 + 2] = (val >> 16) as u8;
            sha_out[i*4 + 3] = (val >> 24) as u8;
        }
        if sha_out == sha_out_ref {
            writeln!(stdout, "MATCH").unwrap();
        } else {
            writeln!(stdout, "MISMATCH").unwrap();
        }
    }
    loop {
        unsafe { asm::wfi(); }
    }
}
