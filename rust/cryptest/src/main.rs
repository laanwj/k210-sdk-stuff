#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![no_std]
#![no_main]

use core::iter;
use hex_literal::hex;
use k210_hal::prelude::*;
use k210_hal::stdout::Stdout;
use k210_hal::pac::Peripherals;
use k210_shared::soc::sleep::usleep;
use k210_shared::soc::sysctl;
use k210_shared::soc::aes::{self, cipher_mode, encrypt_sel};
use k210_shared::soc::sha256::SHA256Ctx;
use k210_shared::timing::clock;
use riscv::asm;
use riscv_rt::entry;
use sha2::{Sha256, Digest};
use aes_soft::block_cipher_trait::generic_array::GenericArray;
use aes_soft::block_cipher_trait::BlockCipher;

struct AESTestVec {
    cipher_mode: cipher_mode,
    key: &'static [u8],
    pt: &'static [u8],
    ct: &'static [u8],
    iv: &'static [u8],
    aad: &'static [u8],
    tag: &'static [u8],
}

struct SHA256TestVec {
    data: &'static [u8],
    hash: [u8; 32],
}

#[entry]
fn main() -> ! {
    let mut p = Peripherals::take().unwrap();
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
        .configure(115_200.bps(), &clocks);
    let (mut tx, _) = serial.split();
    let mut stdout = Stdout(&mut tx);

    usleep(200000);
    writeln!(
        stdout,
        "Init",
    ).unwrap();

    let aes = &mut p.AES;
    let sha256 = &mut p.SHA256;

    // https://boringssl.googlesource.com/boringssl/+/2214/crypto/cipher/cipher_test.txt
    // https://github.com/plenluno/openssl/blob/master/openssl/test/evptests.txt
    // http://csrc.nist.gov/groups/ST/toolkit/BCM/documents/proposedmodes/gcm/gcm-spec.pdf
    for tv in [
        AESTestVec {
            cipher_mode: cipher_mode::ECB,
            key: &hex!("000102030405060708090A0B0C0D0E0F"),
            pt: &hex!("00112233445566778899AABBCCDDEEFF"),
            ct: &hex!("69C4E0D86A7B0430D8CDB78070B4C55A"),
            iv: &hex!(""),
            aad: &hex!(""),
            tag: &hex!(""),
        },
        AESTestVec {
            cipher_mode: cipher_mode::ECB,
            key: &hex!("000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F"),
            pt: &hex!("00112233445566778899AABBCCDDEEFF"),
            ct: &hex!("8EA2B7CA516745BFEAFC49904B496089"),
            iv: &hex!(""),
            aad: &hex!(""),
            tag: &hex!(""),
        },
        AESTestVec {
            cipher_mode: cipher_mode::ECB,
            key: &hex!("2B7E151628AED2A6ABF7158809CF4F3C"),
            pt: &hex!("6BC1BEE22E409F96E93D7E117393172A"),
            ct: &hex!("3AD77BB40D7A3660A89ECAF32466EF97"),
            iv: &hex!(""),
            aad: &hex!(""),
            tag: &hex!(""),
        },
        AESTestVec {
            cipher_mode: cipher_mode::ECB,
            key: &hex!("2B7E151628AED2A6ABF7158809CF4F3C"),
            pt: &hex!("AE2D8A571E03AC9C9EB76FAC45AF8E51"),
            ct: &hex!("F5D3D58503B9699DE785895A96FDBAAF"),
            iv: &hex!(""),
            aad: &hex!(""),
            tag: &hex!(""),
        },
        AESTestVec {
            cipher_mode: cipher_mode::ECB,
            key: &hex!("2B7E151628AED2A6ABF7158809CF4F3C"),
            pt: &hex!("30C81C46A35CE411E5FBC1191A0A52EF"),
            ct: &hex!("43B1CD7F598ECE23881B00E3ED030688"),
            iv: &hex!(""),
            aad: &hex!(""),
            tag: &hex!(""),
        },
        AESTestVec {
            cipher_mode: cipher_mode::ECB,
            key: &hex!("2B7E151628AED2A6ABF7158809CF4F3C"),
            pt: &hex!("F69F2445DF4F9B17AD2B417BE66C3710"),
            ct: &hex!("7B0C785E27E8AD3F8223207104725DD4"),
            iv: &hex!(""),
            aad: &hex!(""),
            tag: &hex!(""),
        },
        AESTestVec {
            cipher_mode: cipher_mode::ECB,
            key: &hex!("603DEB1015CA71BE2B73AEF0857D77811F352C073B6108D72D9810A30914DFF4"),
            pt: &hex!("6BC1BEE22E409F96E93D7E117393172A"),
            ct: &hex!("F3EED1BDB5D2A03C064B5A7E3DB181F8"),
            iv: &hex!(""),
            aad: &hex!(""),
            tag: &hex!(""),
        },
        AESTestVec {
            cipher_mode: cipher_mode::ECB,
            key: &hex!("603DEB1015CA71BE2B73AEF0857D77811F352C073B6108D72D9810A30914DFF4"),
            pt: &hex!("AE2D8A571E03AC9C9EB76FAC45AF8E51"),
            ct: &hex!("591CCB10D410ED26DC5BA74A31362870"),
            iv: &hex!(""),
            aad: &hex!(""),
            tag: &hex!(""),
        },
        AESTestVec {
            cipher_mode: cipher_mode::ECB,
            key: &hex!("603DEB1015CA71BE2B73AEF0857D77811F352C073B6108D72D9810A30914DFF4"),
            pt: &hex!("30C81C46A35CE411E5FBC1191A0A52EF"),
            ct: &hex!("B6ED21B99CA6F4F9F153E7B1BEAFED1D"),
            iv: &hex!(""),
            aad: &hex!(""),
            tag: &hex!(""),
        },
        AESTestVec {
            cipher_mode: cipher_mode::ECB,
            key: &hex!("603DEB1015CA71BE2B73AEF0857D77811F352C073B6108D72D9810A30914DFF4"),
            pt: &hex!("F69F2445DF4F9B17AD2B417BE66C3710"),
            ct: &hex!("23304B7A39F9F3FF067D8D8F9E24ECC7"),
            iv: &hex!(""),
            aad: &hex!(""),
            tag: &hex!(""),
        },
        AESTestVec {
            cipher_mode: cipher_mode::ECB,
            key: &hex!("8e73b0f7da0e6452c810f32b809079e562f8ead2522c6b7b"),
            pt: &hex!("6bc1bee22e409f96e93d7e117393172a"),
            ct: &hex!("bd334f1d6e45f25ff712a214571fa5cc"),
            iv: &hex!(""),
            aad: &hex!(""),
            tag: &hex!(""),
        },
        AESTestVec {
            cipher_mode: cipher_mode::ECB,
            key: &hex!("8e73b0f7da0e6452c810f32b809079e562f8ead2522c6b7b"),
            pt: &hex!("ae2d8a571e03ac9c9eb76fac45af8e51"),
            ct: &hex!("974104846d0ad3ad7734ecb3ecee4eef"),
            iv: &hex!(""),
            aad: &hex!(""),
            tag: &hex!(""),
        },
        AESTestVec {
            cipher_mode: cipher_mode::ECB,
            key: &hex!("8e73b0f7da0e6452c810f32b809079e562f8ead2522c6b7b"),
            pt: &hex!("30c81c46a35ce411e5fbc1191a0a52ef"),
            ct: &hex!("ef7afd2270e2e60adce0ba2face6444e"),
            iv: &hex!(""),
            aad: &hex!(""),
            tag: &hex!(""),
        },
        AESTestVec {
            cipher_mode: cipher_mode::ECB,
            key: &hex!("8e73b0f7da0e6452c810f32b809079e562f8ead2522c6b7b"),
            pt: &hex!("f69f2445df4f9b17ad2b417be66c3710"),
            ct: &hex!("9a4b41ba738d6c72fb16691603c18e0e"),
            iv: &hex!(""),
            aad: &hex!(""),
            tag: &hex!(""),
        },
        AESTestVec {
            cipher_mode: cipher_mode::CBC,
            key: &hex!("2B7E151628AED2A6ABF7158809CF4F3C"),
            pt: &hex!("6BC1BEE22E409F96E93D7E117393172A"),
            ct: &hex!("7649ABAC8119B246CEE98E9B12E9197D"),
            iv: &hex!("000102030405060708090A0B0C0D0E0F"),
            aad: &hex!(""),
            tag: &hex!(""),
        },
        AESTestVec {
            cipher_mode: cipher_mode::CBC,
            key: &hex!("2B7E151628AED2A6ABF7158809CF4F3C"),
            pt: &hex!("AE2D8A571E03AC9C9EB76FAC45AF8E51"),
            ct: &hex!("5086CB9B507219EE95DB113A917678B2"),
            iv: &hex!("7649ABAC8119B246CEE98E9B12E9197D"),
            aad: &hex!(""),
            tag: &hex!(""),
        },
        AESTestVec {
            cipher_mode: cipher_mode::CBC,
            key: &hex!("2B7E151628AED2A6ABF7158809CF4F3C"),
            pt: &hex!("30C81C46A35CE411E5FBC1191A0A52EF"),
            ct: &hex!("73BED6B8E3C1743B7116E69E22229516"),
            iv: &hex!("5086CB9B507219EE95DB113A917678B2"),
            aad: &hex!(""),
            tag: &hex!(""),
        },
        AESTestVec {
            cipher_mode: cipher_mode::CBC,
            key: &hex!("2B7E151628AED2A6ABF7158809CF4F3C"),
            pt: &hex!("F69F2445DF4F9B17AD2B417BE66C3710"),
            ct: &hex!("3FF1CAA1681FAC09120ECA307586E1A7"),
            iv: &hex!("73BED6B8E3C1743B7116E69E22229516"),
            aad: &hex!(""),
            tag: &hex!(""),
        },
        AESTestVec {
            cipher_mode: cipher_mode::CBC,
            key: &hex!("603DEB1015CA71BE2B73AEF0857D77811F352C073B6108D72D9810A30914DFF4"),
            pt: &hex!("6BC1BEE22E409F96E93D7E117393172A"),
            ct: &hex!("F58C4C04D6E5F1BA779EABFB5F7BFBD6"),
            iv: &hex!("000102030405060708090A0B0C0D0E0F"),
            aad: &hex!(""),
            tag: &hex!(""),
        },
        AESTestVec {
            cipher_mode: cipher_mode::CBC,
            key: &hex!("603DEB1015CA71BE2B73AEF0857D77811F352C073B6108D72D9810A30914DFF4"),
            pt: &hex!("AE2D8A571E03AC9C9EB76FAC45AF8E51"),
            ct: &hex!("9CFC4E967EDB808D679F777BC6702C7D"),
            iv: &hex!("F58C4C04D6E5F1BA779EABFB5F7BFBD6"),
            aad: &hex!(""),
            tag: &hex!(""),
        },
        AESTestVec {
            cipher_mode: cipher_mode::CBC,
            key: &hex!("603DEB1015CA71BE2B73AEF0857D77811F352C073B6108D72D9810A30914DFF4"),
            pt: &hex!("30C81C46A35CE411E5FBC1191A0A52EF"),
            ct: &hex!("39F23369A9D9BACFA530E26304231461"),
            iv: &hex!("9CFC4E967EDB808D679F777BC6702C7D"),
            aad: &hex!(""),
            tag: &hex!(""),
        },
        AESTestVec {
            cipher_mode: cipher_mode::CBC,
            key: &hex!("603DEB1015CA71BE2B73AEF0857D77811F352C073B6108D72D9810A30914DFF4"),
            pt: &hex!("F69F2445DF4F9B17AD2B417BE66C3710"),
            ct: &hex!("B2EB05E2C39BE9FCDA6C19078C6A9D1B"),
            iv: &hex!("39F23369A9D9BACFA530E26304231461"),
            aad: &hex!(""),
            tag: &hex!(""),
        },
        AESTestVec {
            cipher_mode: cipher_mode::GCM,
            key: &hex!("e98b72a9881a84ca6b76e0f43e68647a"),
            pt: &hex!("28286a321293253c3e0aa2704a278032"),
            ct: &hex!("5a3c1cf1985dbb8bed818036fdd5ab42"),
            iv: &hex!("8b23299fde174053f3d652ba"),
            aad: &hex!(""),
            tag: &hex!("23c7ab0f952b7091cd324835043b5eb5"),
        },
        AESTestVec {
            cipher_mode: cipher_mode::GCM,
            key: &hex!("816e39070410cf2184904da03ea5075a"),
            pt: &hex!("ecafe96c67a1646744f1c891f5e69427"),
            ct: &hex!("552ebe012e7bcf90fcef712f8344e8f1"),
            iv: &hex!("32c367a3362613b27fc3e67e"),
            aad: &hex!("f2a30728ed874ee02983c294435d3c16"),
            tag: &hex!("ecaae9fc68276a45ab0ca3cb9dd9539f"),
        },
        AESTestVec {
            cipher_mode: cipher_mode::GCM,
            key: &hex!("95bcde70c094f04e3dd8259cafd88ce8"),
            pt: &hex!("32f51e837a9748838925066d69e87180f34a6437e6b396e5643b34cb2ee4f7b1"),
            ct: &hex!("8a023ba477f5b809bddcda8f55e09064d6d88aaec99c1e141212ea5b08503660"),
            iv: &hex!("12cf097ad22380432ff40a5c"),
            aad: &hex!("c783a0cca10a8d9fb8d27d69659463f2"),
            tag: &hex!("562f500dae635d60a769b466e15acd1e"),
        },
        AESTestVec {
            cipher_mode: cipher_mode::GCM,
            key: &hex!("387218b246c1a8257748b56980e50c94"),
            pt: &hex!("48f5b426baca03064554cc2b30"),
            ct: &hex!("cdba9e73eaf3d38eceb2b04a8d"),
            iv: &hex!("dd7e014198672be39f95b69d"),
            aad: &hex!(""),
            tag: &hex!("ecf90f4a47c9c626d6fb2c765d201556"),
        },
        /* tag is wrong when length of plaintext is 0?
        AESTestVec {
            cipher_mode: cipher_mode::GCM,
            key: &hex!("00000000000000000000000000000000"),
            pt: &hex!(""),
            ct: &hex!(""),
            iv: &hex!("000000000000000000000000"),
            aad: &hex!(""),
            tag: &hex!("58e2fccefa7e3061367f1d57a4e7455a"), // ref 00000000fa7e3061367f1d57a4e7455a
        },
        */
        AESTestVec {
            cipher_mode: cipher_mode::GCM,
            key: &hex!("00000000000000000000000000000000"),
            pt: &hex!("00000000000000000000000000000000"),
            ct: &hex!("0388dace60b6a392f328c2b971b2fe78"),
            iv: &hex!("000000000000000000000000"),
            aad: &hex!(""),
            tag: &hex!("ab6e47d42cec13bdf53a67b21257bddf"),
        },
        AESTestVec {
            cipher_mode: cipher_mode::GCM,
            key: &hex!("feffe9928665731c6d6a8f9467308308"),
            pt: &hex!("d9313225f88406e5a55909c5aff5269a86a7a9531534f7da2e4c303d8a318a721c3c0c95956809532fcf0e2449a6b525b16aedf5aa0de657ba637b391aafd255"),
            ct: &hex!("42831ec2217774244b7221b784d0d49ce3aa212f2c02a4e035c17e2329aca12e21d514b25466931c7d8f6a5aac84aa051ba30b396a0aac973d58e091473f5985"),
            iv: &hex!("cafebabefacedbaddecaf888"),
            aad: &hex!(""),
            tag: &hex!("4d5c2af327cd64a62cf35abd2ba6fab4"),
        },
        AESTestVec {
            cipher_mode: cipher_mode::GCM,
            key: &hex!("feffe9928665731c6d6a8f9467308308"),
            pt: &hex!("d9313225f88406e5a55909c5aff5269a86a7a9531534f7da2e4c303d8a318a721c3c0c95956809532fcf0e2449a6b525b16aedf5aa0de657ba637b391aafd255"),
            ct: &hex!("42831ec2217774244b7221b784d0d49ce3aa212f2c02a4e035c17e2329aca12e21d514b25466931c7d8f6a5aac84aa051ba30b396a0aac973d58e091473f5985"),
            iv: &hex!("cafebabefacedbaddecaf888"),
            aad: &hex!(""),
            tag: &hex!("4d5c2af327cd64a62cf35abd2ba6fab4"),
        },
        AESTestVec {
            cipher_mode: cipher_mode::GCM,
            key: &hex!("feffe9928665731c6d6a8f9467308308"),
            pt: &hex!("d9313225f88406e5a55909c5aff5269a86a7a9531534f7da2e4c303d8a318a721c3c0c95956809532fcf0e2449a6b525b16aedf5aa0de657ba637b39"),
            ct: &hex!("42831ec2217774244b7221b784d0d49ce3aa212f2c02a4e035c17e2329aca12e21d514b25466931c7d8f6a5aac84aa051ba30b396a0aac973d58e091"),
            iv: &hex!("cafebabefacedbaddecaf888"),
            aad: &hex!("feedfacedeadbeeffeedfacedeadbeefabaddad2"),
            tag: &hex!("5bc94fbc3221a5db94fae95ae7121a47"),
        },
        /*
        AESTestVec {
            cipher_mode: cipher_mode::GCM,
            key: &hex!("0000000000000000000000000000000000000000000000000000000000000000"),
            pt: &hex!(""),
            ct: &hex!(""),
            iv: &hex!("000000000000000000000000"),
            aad: &hex!(""),
            tag: &hex!("530f8afbc74536b9a963b4f1c4cb738b"),
        },
        */
        AESTestVec {
            cipher_mode: cipher_mode::GCM,
            key: &hex!("0000000000000000000000000000000000000000000000000000000000000000"),
            pt: &hex!("00000000000000000000000000000000"),
            ct: &hex!("cea7403d4d606b6e074ec5d3baf39d18"),
            iv: &hex!("000000000000000000000000"),
            aad: &hex!(""),
            tag: &hex!("d0d1c8a799996bf0265b98b5d48ab919"),
        },
        AESTestVec {
            cipher_mode: cipher_mode::GCM,
            key: &hex!("feffe9928665731c6d6a8f9467308308feffe9928665731c6d6a8f9467308308"),
            pt: &hex!("d9313225f88406e5a55909c5aff5269a86a7a9531534f7da2e4c303d8a318a721c3c0c95956809532fcf0e2449a6b525b16aedf5aa0de657ba637b39"),
            ct: &hex!("522dc1f099567d07f47f37a32a84427d643a8cdcbfe5c0c97598a2bd2555d1aa8cb08e48590dbb3da7b08b1056828838c5f61e6393ba7a0abcc9f662"),
            iv: &hex!("cafebabefacedbaddecaf888"),
            aad: &hex!("feedfacedeadbeeffeedfacedeadbeefabaddad2"),
            tag: &hex!("76fc6ece0f4e1768cddf8853bb2d551b"),
        },
        /*
        // 128 bytes aad
        AESTestVec {
            cipher_mode: cipher_mode::GCM,
            key: &hex!("00000000000000000000000000000000"),
            pt: &hex!(""),
            ct: &hex!(""),
            iv: &hex!("000000000000000000000000"),
            aad: &hex!("d9313225f88406e5a55909c5aff5269a86a7a9531534f7da2e4c303d8a318a721c3c0c95956809532fcf0e2449a6b525b16aedf5aa0de657ba637b391aafd255522dc1f099567d07f47f37a32a84427d643a8cdcbfe5c0c97598a2bd2555d1aa8cb08e48590dbb3da7b08b1056828838c5f61e6393ba7a0abcc9f662898015ad"),
            tag: &hex!("5fea793a2d6f974d37e68e0cb8ff9492"),
        },
        */
        // 48 bytes plaintext
        AESTestVec {
            cipher_mode: cipher_mode::GCM,
            key: &hex!("00000000000000000000000000000000"),
            pt: &hex!("000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"),
            ct: &hex!("0388dace60b6a392f328c2b971b2fe78f795aaab494b5923f7fd89ff948bc1e0200211214e7394da2089b6acd093abe0"),
            iv: &hex!("000000000000000000000000"),
            aad: &hex!(""),
            tag: &hex!("9dd0a376b08e40eb00c35f29f9ea61a4"),
        },
        // 80 bytes plaintext
        AESTestVec {
            cipher_mode: cipher_mode::GCM,
            key: &hex!("00000000000000000000000000000000"),
            pt: &hex!("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"),
            ct: &hex!("0388dace60b6a392f328c2b971b2fe78f795aaab494b5923f7fd89ff948bc1e0200211214e7394da2089b6acd093abe0c94da219118e297d7b7ebcbcc9c388f28ade7d85a8ee35616f7124a9d5270291"),
            iv: &hex!("000000000000000000000000"),
            aad: &hex!(""),
            tag: &hex!("98885a3a22bd4742fe7b72172193b163"),
        },
        // 128 bytes plaintext
        AESTestVec {
            cipher_mode: cipher_mode::GCM,
            key: &hex!("00000000000000000000000000000000"),
            pt: &hex!("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"),
            ct: &hex!("0388dace60b6a392f328c2b971b2fe78f795aaab494b5923f7fd89ff948bc1e0200211214e7394da2089b6acd093abe0c94da219118e297d7b7ebcbcc9c388f28ade7d85a8ee35616f7124a9d527029195b84d1b96c690ff2f2de30bf2ec89e00253786e126504f0dab90c48a30321de3345e6b0461e7c9e6c6b7afedde83f40"),
            iv: &hex!("000000000000000000000000"),
            aad: &hex!(""),
            tag: &hex!("cac45f60e31efd3b5a43b98a22ce1aa1"),
        },
    ].iter() {
        let mut ct_out = [0u8; 128];
        let mut tag_out = [0u8; 16];

        write!(stdout, "AES{}{}: ", tv.key.len()*8,
            match tv.cipher_mode {
                cipher_mode::ECB => "ECB",
                cipher_mode::CBC => "CBC",
                cipher_mode::GCM => "GCM",
            }).unwrap();

        // copy in tag for verification check
        tag_out[0..tv.tag.len()].copy_from_slice(tv.tag);
        let vs = aes::run(
            aes,
            tv.cipher_mode,
            encrypt_sel::ENCRYPTION,
            tv.key,
            tv.iv,
            tv.aad,
            tv.pt,
            &mut ct_out,
            Some(&mut tag_out),
        );

        if &ct_out[0..tv.ct.len()] == tv.ct {
            write!(stdout, "MATCH").unwrap();
        } else {
            write!(stdout, "MISMATCH").unwrap();
        }

        write!(stdout, " ").unwrap();

        if tv.cipher_mode == cipher_mode::GCM {
            let vs = vs.unwrap();
            if vs {
                write!(stdout, "HWTAGMATCH ").unwrap();
            } else {
                write!(stdout, "HWTAGMISMATCH ").unwrap();
            }
            if &tag_out[0..tv.tag.len()] == tv.tag {
                write!(stdout, "TAGMATCH").unwrap();
            } else {
                write!(stdout, "TAGMISMATCH ").unwrap();
                for v in &tag_out[0..tv.tag.len()] {
                    write!(stdout, "{:02x}", v).unwrap();
                }
                write!(stdout, " ref: ").unwrap();
                for v in tv.tag {
                    write!(stdout, "{:02x}", v).unwrap();
                }
            }
        }

        let mut pt_out = [0u8; 128];
        let mut tag_out2 = [0u8; 16];
        // copy in tag for verification check
        tag_out2[0..tv.tag.len()].copy_from_slice(tv.tag);
        let vs = aes::run(
            aes,
            tv.cipher_mode,
            encrypt_sel::DECRYPTION,
            tv.key,
            tv.iv,
            tv.aad,
            &ct_out[0..tv.ct.len()],
            &mut pt_out,
            Some(&mut tag_out2),
        );

        write!(stdout, " ").unwrap();
        if &pt_out[0..tv.pt.len()] == tv.pt {
            write!(stdout, "MATCH").unwrap();
        } else {
            write!(stdout, "MISMATCH").unwrap();
        }
        write!(stdout, " ").unwrap();
        if tv.cipher_mode == cipher_mode::GCM {
            let vs = vs.unwrap();
            if vs {
                write!(stdout, "HWTAGMATCH ").unwrap();
            } else {
                write!(stdout, "HWTAGMISMATCH ").unwrap();
            }
            if &tag_out2[0..tv.tag.len()] == tv.tag {
                write!(stdout, "TAGMATCH").unwrap();
            } else {
                write!(stdout, "TAGMISMATCH ").unwrap();
                for v in &tag_out2[0..tv.tag.len()] {
                    write!(stdout, "{:02x}", v).unwrap();
                }
                write!(stdout, " ref: ").unwrap();
                for v in tv.tag {
                    write!(stdout, "{:02x}", v).unwrap();
                }
            }
        }

        writeln!(stdout).unwrap();
    }

    // https://www.di-mgt.com.au/sha_testvectors.html
    for tv in &[
        SHA256TestVec {
            data: b"",
            hash: hex!("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
        },
        SHA256TestVec {
            data: b"abc",
            hash: hex!("ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad")
        },
        SHA256TestVec {
            data: b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
            hash: hex!("248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1")
        },
        SHA256TestVec {
            data: b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu",
            hash: hex!("cf5b16a778af8380036ce59e7b0492370b249b11e8f07a51afac45037afee9d1")
        },
    ] {
        write!(stdout, "SHA256: ").unwrap();
        let mut sha = SHA256Ctx::new(sha256, tv.data.len());
        sha.update(&tv.data[..]);
        let sha_out = sha.finish();
        if sha_out == tv.hash {
            writeln!(stdout, "MATCH").unwrap();
        } else {
            writeln!(stdout, "MISMATCH").unwrap();
        }
    }

    // Very long test vector
    // Input message: one million (1,000,000) repetitions of the character "a" (0x61).
    {
        let expected = hex!("cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0");
        let size = 1_000_000;
        write!(stdout, "SHA256 ({} bytes): ", size).unwrap();
        let mut sha = SHA256Ctx::new(sha256, size);
        sha.update(iter::repeat(&b'a').take(size));
        let sha_out = sha.finish();
        if sha_out == expected {
            writeln!(stdout, "MATCH").unwrap();
        } else {
            writeln!(stdout, "MISMATCH").unwrap();
        }
    }

    // "abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmno" repeated 65,535 times
    // (this is shorter than the given test vector as it is the maximum that the SHA256 engine
    // supports, 65536 SHA blocks including padding)
    {
        let expected = hex!("929156a9422e05b71655509e8e9e7292d65d540a7342c94df3e121cedd407dfe");
        let s = b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmno";
        let s_u32 = [0x64636261, 0x68676665, 0x65646362, 0x69686766, 0x66656463, 0x6a696867, 0x67666564, 0x6b6a6968, 0x68676665, 0x6c6b6a69, 0x69686766, 0x6d6c6b6a, 0x6a696867, 0x6e6d6c6b, 0x6b6a6968, 0x6f6e6d6c];
        // let size = 16_777_216 * s.len();
        let size = 65_535 * s.len();

        write!(stdout, "SHA256 hw ({} bytes): ", size).unwrap();
        let time_start = clock();
        let mut sha = SHA256Ctx::new(sha256, size);
        sha.update(s.iter().cycle().take(size));
        let sha_out = sha.finish();
        let time_end = clock();
        if sha_out == expected {
            write!(stdout, "MATCH").unwrap();
        } else {
            write!(stdout, "MISMATCH").unwrap();
        }
        write!(stdout, " ({} kB/s)", (size as u64) * 1_000 / (time_end - time_start)).unwrap();
        writeln!(stdout).unwrap();

        write!(stdout, "SHA256 hw, 32bit ({} bytes): ", size).unwrap();
        let time_start = clock();
        let mut sha = SHA256Ctx::new(sha256, size);
        sha.update32(s_u32.iter().cycle().take(size / 4));
        let sha_out = sha.finish();
        let time_end = clock();
        if sha_out == expected {
            write!(stdout, "MATCH").unwrap();
        } else {
            write!(stdout, "MISMATCH").unwrap();
        }
        write!(stdout, " ({} kB/s)", (size as u64) * 1_000 / (time_end - time_start)).unwrap();
        writeln!(stdout).unwrap();

        // Yet another thing to try would be DMA?

        // Software
        write!(stdout, "SHA256 sw ({} bytes): ", size).unwrap();
        let time_start = clock();
        let mut hasher = Sha256::new();
        for _ in 0..65_535 {
            hasher.input(&s[..]);
        }
        let time_end = clock();
        if hasher.result()[..] == expected {
            write!(stdout, "MATCH").unwrap();
        } else {
            write!(stdout, "MISMATCH").unwrap();
        }
        write!(stdout, " ({} kB/s)", (size as u64) * 1_000 / (time_end - time_start)).unwrap();
        writeln!(stdout).unwrap();
    }

    // Try iterator-based AES
    // let indata = [0, 0, 0, 0, 0, 0, 0, 0];
    let size = 256 * 1024;
    write!(stdout, "AES128 hw, 32bit ({} bytes): ", size).unwrap();
    let time_start = clock();
    let mut o = aes::run_iter32(
            aes,
            cipher_mode::ECB,
            encrypt_sel::ENCRYPTION,
            &hex!("00000000000000000000000000000000"),
            &hex!(""),
            &hex!(""),
            //indata.iter().copied(),
            iter::repeat(0).take(size / 4),
            size,
        );
    for _ in 0..size / 16 {
        // ;
        // write!(stdout, "{:08x} ", o.next().unwrap()).unwrap();
        assert!(o.next().unwrap() == 0xd44be966);
        assert!(o.next().unwrap() == 0x3b2c8aef);
        assert!(o.next().unwrap() == 0x59fa4c88);
        assert!(o.next().unwrap() == 0x2e2b34ca);
    }
    // writeln!(stdout).unwrap();
    core::mem::drop(o);
    let time_end = clock();
    writeln!(stdout, "({} kB/s)", (size as u64) * 1_000 / (time_end - time_start)).unwrap();

    write!(stdout, "AES128 sw ({} bytes): ", size).unwrap();
    let time_start = clock();
    let key = GenericArray::from_slice(&[0u8; 16]);
    let cipher = aes_soft::Aes128::new(&key);
    let expect = GenericArray::clone_from_slice(&[0x66, 0xe9, 0x4b, 0xd4, 0xef, 0x8a, 0x2c, 0x3b, 0x88, 0x4c, 0xfa, 0x59, 0xca, 0x34, 0x2b, 0x2e]);
    for _ in 0..size / 16 { // per block
        let mut block = GenericArray::clone_from_slice(&[0u8; 16]);
        cipher.encrypt_block(&mut block);
        // writeln!(stdout, "{:?}", block).unwrap();
        assert!(block == expect);
    }
    let time_end = clock();
    writeln!(stdout, "({} kB/s)", (size as u64) * 1_000 / (time_end - time_start)).unwrap();

    loop {
        unsafe { asm::wfi(); }
    }
}
