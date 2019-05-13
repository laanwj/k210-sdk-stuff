use core::convert::TryInto;
use core::result::Result;
use k210_hal::pac;

use crate::soc::sysctl;

// TODO parametrize type, other I2C than I2C0
fn clk_init()
{
    sysctl::clock_enable(sysctl::clock::I2C0);
    sysctl::clock_set_threshold(sysctl::threshold::I2C0, 3);
}

pub fn init(slave_address: u16, address_width: u32, i2c_clk: u32)
{
    assert!(address_width == 7 || address_width == 10);

    clk_init();

    let v_i2c_freq = sysctl::clock_get_freq(sysctl::clock::I2C0);
    let v_period_clk_cnt = v_i2c_freq / i2c_clk / 2;
    let v_period_clk_cnt: u16 = v_period_clk_cnt.try_into().unwrap();
    let v_period_clk_cnt = if v_period_clk_cnt == 0 { 1 } else { v_period_clk_cnt };

    unsafe {
        let ptr = pac::I2C0::ptr();
        (*ptr).enable.write(|w| w.bits(0));
        (*ptr).con.write(|w| w.master_mode().bit(true)
                             .slave_disable().bit(true)
                             .restart_en().bit(true)
                             .addr_slave_width().bit(address_width == 10) // TODO variant
                             .speed().bits(1)); // TODO variant
        (*ptr).ss_scl_hcnt.write(|w| w.count().bits(v_period_clk_cnt));
        (*ptr).ss_scl_lcnt.write(|w| w.count().bits(v_period_clk_cnt));
        (*ptr).tar.write(|w| w.address().bits(slave_address));
        (*ptr).intr_mask.write(|w| w.bits(0));
        (*ptr).dma_cr.write(|w| w.bits(0x3));
        (*ptr).dma_rdlr.write(|w| w.bits(0));
        (*ptr).dma_tdlr.write(|w| w.bits(4));
        (*ptr).enable.write(|w| w.enable().bit(true));
    }
}

pub fn recv_data(send_buf: &[u8], receive_buf: &mut [u8]) -> Result<(), ()>
{
    unsafe {
        let ptr = pac::I2C0::ptr();
        let mut txi = 0;
        let mut tx_left = send_buf.len();
        while tx_left != 0 {
            let fifo_len = 8 - ((*ptr).txflr.read().bits() as usize);
            let fifo_len = if tx_left < fifo_len { tx_left } else { fifo_len };
            for _ in 0..fifo_len {
                (*ptr).data_cmd.write(|w| w.data().bits(send_buf[txi]));
                txi += 1;
            }
            if (*ptr).tx_abrt_source.read().bits() != 0 {
                return Err(());
            }
            tx_left -= fifo_len;
        }

        let mut cmd_count = receive_buf.len();
        let mut rx_left = receive_buf.len();
        let mut rxi = 0;
        while cmd_count != 0 || rx_left != 0 {
            /* XXX this is a kind of strange construction, sanity check */
            let fifo_len = (*ptr).rxflr.read().bits() as usize;
            let fifo_len = if rx_left < fifo_len { rx_left } else { fifo_len };
            for _ in 0..fifo_len {
                receive_buf[rxi] = (*ptr).data_cmd.read().data().bits();
                rxi += 1;
            }
            rx_left -= fifo_len;

            /* send 0x100 for every byte that we want to receive */
            let fifo_len = 8 - (*ptr).txflr.read().bits() as usize;
            let fifo_len = if cmd_count < fifo_len { cmd_count } else { fifo_len };
            for _ in 0..fifo_len {
                (*ptr).data_cmd.write(|w| w.cmd().bit(true));
            }
            if (*ptr).tx_abrt_source.read().bits() != 0 {
                return Err(());
            }
            cmd_count -= fifo_len;
        }
    }
    Ok(())
}

