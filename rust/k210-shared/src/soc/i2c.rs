//! I2C peripherals
use core::cmp;
use core::convert::TryInto;
use core::ops::Deref;
use core::result::Result;
use k210_hal::pac::{I2C0,I2C1,I2C2,i2c0};

use crate::soc::sysctl;

/// Trait for generalizing over I2C0-2
pub trait I2CExt: Deref<Target = i2c0::RegisterBlock> + Sized {
    #[doc(hidden)]
    const CLK: sysctl::clock;
    #[doc(hidden)]
    const DIV: sysctl::threshold;
    #[doc(hidden)]
    const RESET: sysctl::reset;

    /// Constrains I2C peripheral so it plays nicely with the other abstractions
    fn constrain(self) -> I2CImpl<Self>;
}

impl I2CExt for I2C0 {
    const CLK: sysctl::clock = sysctl::clock::I2C0;
    const DIV: sysctl::threshold = sysctl::threshold::I2C0;
    const RESET: sysctl::reset = sysctl::reset::I2C0;

    fn constrain(self) -> I2CImpl<Self> { I2CImpl::<Self> { i2c: self } }
}
impl I2CExt for I2C1 {
    const CLK: sysctl::clock = sysctl::clock::I2C1;
    const DIV: sysctl::threshold = sysctl::threshold::I2C1;
    const RESET: sysctl::reset = sysctl::reset::I2C1;

    fn constrain(self) -> I2CImpl<Self> { I2CImpl::<Self> { i2c: self } }
}
impl I2CExt for I2C2 {
    const CLK: sysctl::clock = sysctl::clock::I2C2;
    const DIV: sysctl::threshold = sysctl::threshold::I2C2;
    const RESET: sysctl::reset = sysctl::reset::I2C2;

    fn constrain(self) -> I2CImpl<Self> { I2CImpl::<Self> { i2c: self } }
}

pub struct I2CImpl<IF> {
    i2c: IF,
}

pub trait I2C {
    fn init(&self, slave_address: u16, address_width: u32, i2c_clk: u32);
    fn recv_data(&self, send_buf: &[u8], receive_buf: &mut [u8]) -> Result<(), ()>;
    fn send_data(&self, send_buf: &[u8]) -> Result<(), ()>;
}

impl<IF: I2CExt> I2C for I2CImpl<IF> {
    fn init(&self, slave_address: u16, address_width: u32, i2c_clk: u32) {
        // sets up a fixed clock divide by (3+1)*2=8
        sysctl::clock_enable(IF::CLK);
        sysctl::clock_set_threshold(IF::DIV, 3);
        sysctl::reset(IF::RESET);

        let v_i2c_freq = sysctl::clock_get_freq(IF::CLK);
        let v_period_clk_cnt = v_i2c_freq / i2c_clk / 2;
        let v_period_clk_cnt: u16 = v_period_clk_cnt.try_into().unwrap();
        let v_period_clk_cnt = cmp::max(v_period_clk_cnt, 1);

        use i2c0::con::{ADDR_SLAVE_WIDTH_A,SPEED_A};
        let v_width = match address_width {
            7 => ADDR_SLAVE_WIDTH_A::B7,
            10 => ADDR_SLAVE_WIDTH_A::B10,
            _ => panic!("unsupported address width"),
        };
        unsafe {
            self.i2c.enable.write(|w| w.bits(0));
            self.i2c.con.write(|w| w.master_mode().bit(true)
                                 .slave_disable().bit(true)
                                 .restart_en().bit(true)
                                 .addr_slave_width().variant(v_width)
                                 .speed().variant(SPEED_A::FAST));
            self.i2c.ss_scl_hcnt.write(|w| w.count().bits(v_period_clk_cnt));
            self.i2c.ss_scl_lcnt.write(|w| w.count().bits(v_period_clk_cnt));
            self.i2c.tar.write(|w| w.address().bits(slave_address));
            self.i2c.intr_mask.write(|w| w.bits(0));
            self.i2c.dma_cr.write(|w| w.bits(0x3));
            self.i2c.dma_rdlr.write(|w| w.bits(0));
            self.i2c.dma_tdlr.write(|w| w.bits(4));
            self.i2c.enable.write(|w| w.enable().bit(true));
        }
    }

    fn recv_data(&self, send_buf: &[u8], receive_buf: &mut [u8]) -> Result<(), ()> {
        unsafe {
            let mut txi = 0;
            let mut tx_left = send_buf.len();
            while tx_left != 0 {
                let fifo_len = 8 - (self.i2c.txflr.read().bits() as usize);
                let fifo_len = cmp::min(tx_left, fifo_len);
                for _ in 0..fifo_len {
                    self.i2c.data_cmd.write(|w| w.data().bits(send_buf[txi]));
                    txi += 1;
                }
                if self.i2c.tx_abrt_source.read().bits() != 0 {
                    return Err(());
                }
                tx_left -= fifo_len;
            }

            let mut cmd_count = receive_buf.len();
            let mut rx_left = receive_buf.len();
            let mut rxi = 0;
            while cmd_count != 0 || rx_left != 0 {
                /* XXX this is a kind of strange construction, sanity check */
                let fifo_len = self.i2c.rxflr.read().bits() as usize;
                let fifo_len = cmp::min(rx_left, fifo_len);
                for _ in 0..fifo_len {
                    receive_buf[rxi] = self.i2c.data_cmd.read().data().bits();
                    rxi += 1;
                }
                rx_left -= fifo_len;

                /* send 0x100 for every byte that we want to receive */
                let fifo_len = 8 - self.i2c.txflr.read().bits() as usize;
                let fifo_len = cmp::min(cmd_count, fifo_len);
                for _ in 0..fifo_len {
                    self.i2c.data_cmd.write(|w| w.cmd().bit(true));
                }
                if self.i2c.tx_abrt_source.read().bits() != 0 {
                    return Err(());
                }
                cmd_count -= fifo_len;
            }
        }
        Ok(())
    }

    fn send_data(&self, send_buf: &[u8]) -> Result<(), ()> {
        unsafe {
            let mut txi = 0;
            let mut tx_left = send_buf.len();

            // Clear TX abort by reading from clear register
            // Hopefully this is not optimized out
            self.i2c.clr_tx_abrt.read().bits();

            // Send all data that is left, handle errors that occur
            while tx_left != 0 {
                let fifo_len = 8 - (self.i2c.txflr.read().bits() as usize);
                let fifo_len = cmp::min(tx_left, fifo_len);
                for _ in 0..fifo_len {
                    self.i2c.data_cmd.write(|w| w.data().bits(send_buf[txi]));
                    txi += 1;
                }
                if self.i2c.tx_abrt_source.read().bits() != 0 {
                    return Err(());
                }
                tx_left -= fifo_len;
            }

            // Wait for TX succeed
            while self.i2c.status.read().activity().bit() || !self.i2c.status.read().tfe().bit() {
                // NOP
            }

            // Check for errors one last time
            if self.i2c.tx_abrt_source.read().bits() != 0 {
                return Err(());
            }
        }
        Ok(())
    }
}
