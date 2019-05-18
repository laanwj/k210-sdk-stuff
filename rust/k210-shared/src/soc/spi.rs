use core::cmp;
use core::ops::Deref;
use k210_hal::pac;
use pac::{SPI0,SPI1,spi0};
use pac::spi0::ctrlr0;
use pac::spi0::spi_ctrlr0;

use crate::soc::sysctl;

/// Extension trait that constrains SPI peripherals
pub trait SPIExt: Sized {
    /// Constrains SPI peripheral so it plays nicely with the other abstractions
    fn constrain(self) -> SPIImpl<Self>;
}

/// Trait for generalizing over SPI0 and SPI1 (SPI2 is slave-only and SPI3 is !!!special!!!)
pub trait SPI01: Deref<Target = spi0::RegisterBlock> {
    #[doc(hidden)]
    const CLK: sysctl::clock;
    #[doc(hidden)]
    const DIV: sysctl::threshold;
}

impl SPI01 for SPI0 {
    const CLK: sysctl::clock = sysctl::clock::SPI0;
    const DIV: sysctl::threshold = sysctl::threshold::SPI0;
}
impl SPI01 for SPI1 {
    const CLK: sysctl::clock = sysctl::clock::SPI1;
    const DIV: sysctl::threshold = sysctl::threshold::SPI1;
}

impl<SPI: SPI01> SPIExt for SPI {
    fn constrain(self) -> SPIImpl<SPI> {
        SPIImpl::<SPI>::new(self)
    }
}

pub struct SPIImpl<IF> {
    spi: IF,
}

pub trait SPI {
    fn configure(
        &self,
        work_mode: ctrlr0::WORK_MODEW,
        frame_format: ctrlr0::FRAME_FORMATW,
        data_bit_length: u8,
        endian: u32,
        instruction_length: u8,
        address_length: u8,
        wait_cycles: u8,
        instruction_address_trans_mode: spi_ctrlr0::AITMW,
        tmod: ctrlr0::TMODW,
    );
    fn set_clk_rate(&self, spi_clk: u32) -> u32;
    fn send_data<X: Into<u32> + Copy>(&self, chip_select: u32, tx: &[X]);
    fn fill_data(&self, chip_select: u32, value: u32, tx_len: usize);
}

impl<IF: SPI01> SPIImpl<IF> {
    pub fn new(spi: IF) -> Self {
        Self { spi }
    }
}

impl<IF: SPI01> SPI for SPIImpl<IF> {
    /// Configure SPI transaction
    fn configure(
        &self,
        work_mode: ctrlr0::WORK_MODEW,
        frame_format: ctrlr0::FRAME_FORMATW,
        data_bit_length: u8,
        endian: u32,
        instruction_length: u8,
        address_length: u8,
        wait_cycles: u8,
        instruction_address_trans_mode: spi_ctrlr0::AITMW,
        tmod: ctrlr0::TMODW,
    ) {
        assert!(data_bit_length >= 4 && data_bit_length <= 32);
        assert!(wait_cycles < (1 << 5));
        let inst_l: u8 = match instruction_length {
            0 => 0,
            4 => 1,
            8 => 2,
            16 => 3,
            _ => panic!("unhandled intruction length"),
        };

        assert!(address_length % 4 == 0 && address_length <= 60);
        let addr_l: u8 = address_length / 4;

        unsafe {
            self.spi.imr.write(|w| w.bits(0x00));
            self.spi.dmacr.write(|w| w.bits(0x00));
            self.spi.dmatdlr.write(|w| w.bits(0x10));
            self.spi.dmardlr.write(|w| w.bits(0x00));
            self.spi.ser.write(|w| w.bits(0x00));
            self.spi.ssienr.write(|w| w.bits(0x00));
            self.spi.ctrlr0.write(|w| {
                w.work_mode()
                    .variant(work_mode)
                    .tmod()
                    .variant(tmod)
                    .frame_format()
                    .variant(frame_format)
                    .data_length()
                    .bits(data_bit_length - 1)
            });
            self.spi.spi_ctrlr0.write(|w| {
                w.aitm()
                    .variant(instruction_address_trans_mode)
                    .addr_length()
                    .bits(addr_l)
                    .inst_length()
                    .bits(inst_l)
                    .wait_cycles()
                    .bits(wait_cycles)
            });
            self.spi.endian.write(|w| w.bits(endian));
        }
    }

    /// Set SPI clock rate
    fn set_clk_rate(&self, spi_clk: u32) -> u32 {
        sysctl::clock_enable(IF::CLK);
        sysctl::clock_set_threshold(IF::DIV, 0);
        let clock_freq: u32 = sysctl::clock_get_freq(sysctl::clock::SPI0);
        let spi_baudr = clock_freq / spi_clk;
        // Clamp baudrate divider to valid range
        let spi_baudr = cmp::min(cmp::max(spi_baudr, 2), 65534);
        unsafe {
            self.spi.baudr.write(|w| w.bits(spi_baudr));
        }
        clock_freq / spi_baudr
    }

    /// Send arbitrary data
    fn send_data<X: Into<u32> + Copy>(&self, chip_select: u32, tx: &[X]) {
        unsafe {
            self.spi.ser.write(|w| w.bits(1 << chip_select));
            self.spi.ssienr.write(|w| w.bits(0x01));

            // TODO: write this using iterators / slices
            let mut i = 0;
            let mut tx_len = tx.len();
            while tx_len != 0 {
                let fifo_len = (32 - self.spi.txflr.read().bits()) as usize;
                let fifo_len = cmp::min(fifo_len, tx_len);
                for _ in 0..fifo_len {
                    self.spi.dr[0].write(|f| f.bits(tx[i].into()));
                    i += 1;
                }
                tx_len -= fifo_len;
            }

            while (self.spi.sr.read().bits() & 0x05) != 0x04 {
                // IDLE
            }
            self.spi.ser.write(|w| w.bits(0x00));
            self.spi.ssienr.write(|w| w.bits(0x00));
        }
    }

    /// Send repeated data
    fn fill_data(&self, chip_select: u32, value: u32, mut tx_len: usize) {
        unsafe {
            self.spi.ser.write(|w| w.bits(1 << chip_select));
            self.spi.ssienr.write(|w| w.bits(0x01));

            while tx_len != 0 {
                let fifo_len = (32 - self.spi.txflr.read().bits()) as usize;
                let fifo_len = cmp::min(fifo_len, tx_len);
                for _ in 0..fifo_len {
                    self.spi.dr[0].write(|f| f.bits(value));
                }
                tx_len -= fifo_len;
            }

            while (self.spi.sr.read().bits() & 0x05) != 0x04 {
                // IDLE
            }
            self.spi.ser.write(|w| w.bits(0x00));
            self.spi.ssienr.write(|w| w.bits(0x00));
        }
    }
}

