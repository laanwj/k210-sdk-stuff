//! SHA256 peripheral
use core::iter;
use core::sync::atomic::{self, Ordering};
use k210_hal::pac;
use pac::sha256::function_reg_0::ENDIAN_A;

/** SHA256 context for hardware-based computation on K210 SHA256 engine. */
pub struct SHA256Ctx<'a> {
    sha: &'a mut pac::SHA256,
    block: u32,
    ptr: usize,
}

impl <'a> SHA256Ctx<'a> {
    /** Create new SHA256 hardware context. As the peripheral has internal
     * state and supports only one context, this prevents creating multiple concurrent
     * contexts by requiring a mutable reference.
     * Unlike software SHA256, the peripheral needs to know the total number of blocks
     * in advance.`input_len` must match the number of bytes that will be fed using update
     * before calling finish, or the result will likely be incorrect or the engine will hang.
     */
    pub fn new(sha: &'a mut pac::SHA256, input_len: usize) -> SHA256Ctx {
        let num_blocks = (input_len + 64 + 8) / 64;
        // Can hash up to 65536 blocks, this is the largest value that fits into data_cnt
        // (0 = 65536).
        assert!(num_blocks <= 0x10000);
        unsafe {
            sha.num_reg.write(|w|
                w.data_cnt().bits(((input_len + 64 + 8) / 64) as u16));
            sha.function_reg_0.write(|w|
                w.endian().variant(ENDIAN_A::BE)
                 .en().bit(true));
            sha.function_reg_1.write(|w|
                w.dma_en().bit(false));
        }
        SHA256Ctx { sha, block: 0, ptr: 0 }
    }

    /** Update SHA256 computation with new data. */
    pub fn update<'b, X>(&mut self, data: X)
        where X: IntoIterator<Item = &'b u8> {
        let mut block = self.block;
        let mut ptr = self.ptr;
        for &v in data {
            let copy_ofs = ptr % 4;
            block |= (v as u32) << (copy_ofs * 8);
            ptr += 1;

            if copy_ofs == 3 {
                unsafe {
                    while self.sha.function_reg_1.read().fifo_in_full().bit() {
                        atomic::compiler_fence(Ordering::SeqCst)
                    }
                    self.sha.data_in.write(|w| w.bits(block));
                }
                block = 0;
            }
        }
        self.block = block;
        self.ptr = ptr;
    }

    /** Update SHA256 computation with new data (32 bit little-endian, must be four-aligned in
     * the data stream). This is roughly two times faster than byte by byte using `update`.
     */
    pub fn update32<'b, X>(&mut self, data: X)
        where X: IntoIterator<Item = &'b u32> {
        assert!((self.ptr & 3) == 0);
        let mut ptr = self.ptr;
        for &v in data {
            unsafe {
                while self.sha.function_reg_1.read().fifo_in_full().bit() {
                    atomic::compiler_fence(Ordering::SeqCst)
                }
                self.sha.data_in.write(|w| w.bits(v));
            }
            ptr += 4;
        }
        self.ptr = ptr;
    }

    /** Finish up SHA256 computation. */
    pub fn finish(&mut self) -> [u8; 32] {
        let length_pad = ((self.ptr as u64) * 8).to_be_bytes();
        let mut bytes_to_pad = 120 - (self.ptr % 64);
        if bytes_to_pad > 64 {
            bytes_to_pad -= 64;
        }
        self.update(&[0x80]);
        self.update(iter::repeat(&0).take(bytes_to_pad - 1));
        self.update(&length_pad);

        while !self.sha.function_reg_0.read().en().bit() {
            atomic::compiler_fence(Ordering::SeqCst)
        }
        let mut sha_out = [0u8; 32];
        for i in 0..8 {
            let val = self.sha.result[7 - i].read().bits().to_le_bytes();
            sha_out[i*4..i*4+4].copy_from_slice(&val);
        }
        sha_out
    }
}
