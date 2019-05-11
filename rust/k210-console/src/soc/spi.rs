use k210_hal::pac;
use pac::spi0::ctrlr0;
use pac::spi0::spi_ctrlr0;

use crate::soc::sysctl;

pub fn clk_init() {
    sysctl::clock_enable(sysctl::clock::SPI0);
    sysctl::clock_set_threshold(sysctl::threshold::SPI0, 0);
    unsafe {
        (*pac::SPI0::ptr()).baudr.write(|w| w.bits(0x14)); // Set baudrate to some default
    }
}

pub fn init(
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
        let ptr = pac::SPI0::ptr();
        (*ptr).imr.write(|w| w.bits(0x00));
        (*ptr).dmacr.write(|w| w.bits(0x00));
        (*ptr).dmatdlr.write(|w| w.bits(0x10));
        (*ptr).dmardlr.write(|w| w.bits(0x00));
        (*ptr).ser.write(|w| w.bits(0x00));
        (*ptr).ssienr.write(|w| w.bits(0x00));
        (*ptr).ctrlr0.write(|w| {
            w.work_mode()
                .variant(work_mode)
                .tmod()
                .variant(tmod)
                .frame_format()
                .variant(frame_format)
                .data_length()
                .bits(data_bit_length - 1)
        });
        (*ptr).spi_ctrlr0.write(|w| {
            w.aitm()
                .variant(instruction_address_trans_mode)
                .addr_length()
                .bits(addr_l)
                .inst_length()
                .bits(inst_l)
                .wait_cycles()
                .bits(wait_cycles)
        });
        (*ptr).endian.write(|w| w.bits(endian));
    }
}

pub fn set_clk_rate(spi_clk: u32) -> u32 {
    let clock_freq: u32 = sysctl::clock_get_freq(sysctl::clock::SPI0);
    let mut spi_baudr = clock_freq / spi_clk;
    if spi_baudr < 2 {
        spi_baudr = 2;
    } else if spi_baudr > 65534 {
        spi_baudr = 65534;
    }
    unsafe {
        (*pac::SPI0::ptr()).baudr.write(|w| w.bits(spi_baudr));
    }
    clock_freq / spi_baudr
}

pub fn send_data<X: Into<u32> + Copy>(chip_select: u32, tx: &[X]) {
    unsafe {
        let ptr = pac::SPI0::ptr();

        (*ptr).ser.write(|w| w.bits(1 << chip_select));
        (*ptr).ssienr.write(|w| w.bits(0x01));

        // TODO: write this using iterators / slices
        let mut i = 0;
        let mut tx_len = tx.len();
        while tx_len != 0 {
            let mut fifo_len = (32 - (*ptr).txflr.read().bits()) as usize;
            fifo_len = if fifo_len < tx_len { fifo_len } else { tx_len };
            for _ in 0..fifo_len {
                (*ptr).dr[0].write(|f| f.bits(tx[i].into()));
                i += 1;
            }
            tx_len -= fifo_len;
        }

        while ((*ptr).sr.read().bits() & 0x05) != 0x04 {
            // IDLE
        }
        (*ptr).ser.write(|w| w.bits(0x00));
        (*ptr).ssienr.write(|w| w.bits(0x00));
    }
}

pub fn fill_data(chip_select: u32, value: u32, mut tx_len: usize) {
    unsafe {
        let ptr = pac::SPI0::ptr();

        (*ptr).ser.write(|w| w.bits(1 << chip_select));
        (*ptr).ssienr.write(|w| w.bits(0x01));

        while tx_len != 0 {
            let mut fifo_len = (32 - (*ptr).txflr.read().bits()) as usize;
            fifo_len = if fifo_len < tx_len { fifo_len } else { tx_len };
            for _ in 0..fifo_len {
                (*ptr).dr[0].write(|f| f.bits(value));
            }
            tx_len -= fifo_len;
        }

        while ((*ptr).sr.read().bits() & 0x05) != 0x04 {
            // IDLE
        }
        (*ptr).ser.write(|w| w.bits(0x00));
        (*ptr).ssienr.write(|w| w.bits(0x00));
    }
}
