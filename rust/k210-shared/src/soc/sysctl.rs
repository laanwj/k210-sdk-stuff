use k210_hal::pac;

use crate::soc::utils::set_bit;

const SYSCTRL_CLOCK_FREQ_IN0: u32 = 26000000;

#[derive(Copy, Clone)]
pub enum pll {
    PLL0,
    PLL1,
    PLL2,
}

#[derive(Copy, Clone)]
pub enum clock_source {
    IN0,
    PLL0,
    PLL1,
    PLL2,
    ACLK,
}

#[derive(Copy, Clone)]
pub enum clock {
    PLL0,
    PLL1,
    PLL2,
    CPU,
    SRAM0,
    SRAM1,
    APB0,
    APB1,
    APB2,
    ROM,
    DMA,
    AI,
    DVP,
    FFT,
    GPIO,
    SPI0,
    SPI1,
    SPI2,
    SPI3,
    I2S0,
    I2S1,
    I2S2,
    I2C0,
    I2C1,
    I2C2,
    UART1,
    UART2,
    UART3,
    AES,
    FPIOA,
    TIMER0,
    TIMER1,
    TIMER2,
    WDT0,
    WDT1,
    SHA,
    OTP,
    RTC,
    ACLK,
    HCLK,
    IN0,
}

#[derive(Copy, Clone)]
pub enum threshold {
    ACLK,
    APB0,
    APB1,
    APB2,
    SRAM0,
    SRAM1,
    AI,
    DVP,
    ROM,
    SPI0,
    SPI1,
    SPI2,
    SPI3,
    TIMER0,
    TIMER1,
    TIMER2,
    I2S0,
    I2S1,
    I2S2,
    I2S0_M,
    I2S1_M,
    I2S2_M,
    I2C0,
    I2C1,
    I2C2,
    WDT0,
    WDT1,
}

#[derive(Copy, Clone)]
pub enum clock_select {
    PLL0_BYPASS,
    PLL1_BYPASS,
    PLL2_BYPASS,
    PLL2,
    ACLK,
    SPI3,
    TIMER0,
    TIMER1,
    TIMER2,
    SPI3_SAMPLE,
}

#[derive(Copy, Clone)]
pub enum io_power_mode {
    V33,
    V18,
}

#[derive(Copy, Clone)]
pub enum power_bank {
    BANK0 = 0,
    BANK1,
    BANK2,
    BANK3,
    BANK4,
    BANK5,
    BANK6,
    BANK7,
}

fn clock_bus_en(clock: clock, en: bool) {
    /*
     * The timer is under APB0, to prevent apb0_clk_en1 and apb0_clk_en0
     * on same register, we split it to peripheral and central two
     * registers, to protect CPU close apb0 clock accidentally.
     *
     * The apb0_clk_en0 and apb0_clk_en1 have same function,
     * one of them set, the APB0 clock enable.
     */

    /* The APB clock should carefully disable */
    if en {
        match clock {
            /*
             * These peripheral devices are under APB0
             * GPIO, UART1, UART2, UART3, SPI_SLAVE, I2S0, I2S1,
             * I2S2, I2C0, I2C1, I2C2, FPIOA, SHA256, TIMER0,
             * TIMER1, TIMER2
             */
            clock::GPIO
            | clock::SPI2
            | clock::I2S0
            | clock::I2S1
            | clock::I2S2
            | clock::I2C0
            | clock::I2C1
            | clock::I2C2
            | clock::UART1
            | clock::UART2
            | clock::UART3
            | clock::FPIOA
            | clock::TIMER0
            | clock::TIMER1
            | clock::TIMER2
            | clock::SHA => unsafe {
                (*pac::SYSCTL::ptr())
                    .clk_en_cent
                    .modify(|_, w| w.apb0_clk_en().bit(en));
            },

            /*
             * These peripheral devices are under APB1
             * WDT, AES, OTP, DVP, SYSCTL
             */
            clock::AES | clock::WDT0 | clock::WDT1 | clock::OTP | clock::RTC => unsafe {
                (*pac::SYSCTL::ptr())
                    .clk_en_cent
                    .modify(|_, w| w.apb1_clk_en().bit(en));
            },

            /*
             * These peripheral devices are under APB2
             * SPI0, SPI1
             */
            clock::SPI0 | clock::SPI1 => unsafe {
                (*pac::SYSCTL::ptr())
                    .clk_en_cent
                    .modify(|_, w| w.apb2_clk_en().bit(en));
            },

            _ => {}
        }
    }
}

fn clock_device_en(clock: clock, en: bool) {
    unsafe {
        let ptr = pac::SYSCTL::ptr();
        match clock {
            /*
            /*
             * These devices are PLL
             */
            clock::PLL0 => sysctl->pll0.pll_out_en0 = en;
            clock::PLL1 => sysctl->pll1.pll_out_en1 = en;
            clock::PLL2 => sysctl->pll2.pll_out_en2 = en;

            /*
             * These devices are CPU, SRAM, APB bus, ROM, DMA, AI
             */
            clock::CPU => sysctl->clk_en_cent.cpu_clk_en = en;
            clock::SRAM0 => sysctl->clk_en_cent.sram0_clk_en = en;
            clock::SRAM1 => sysctl->clk_en_cent.sram1_clk_en = en;
            clock::APB0 => sysctl->clk_en_cent.apb0_clk_en = en;
            clock::APB1 => sysctl->clk_en_cent.apb1_clk_en = en;
            clock::APB2 => sysctl->clk_en_cent.apb2_clk_en = en;
            clock::ROM => sysctl->clk_en_peri.rom_clk_en = en;
            clock::DMA => sysctl->clk_en_peri.dma_clk_en = en;
            clock::AI => sysctl->clk_en_peri.ai_clk_en = en;
            clock::DVP => sysctl->clk_en_peri.dvp_clk_en = en;
            clock::FFT => sysctl->clk_en_peri.fft_clk_en = en;
            clock::SPI3 => sysctl->clk_en_peri.spi3_clk_en = en;

            /*
             * These peripheral devices are under APB0
             * GPIO, UART1, UART2, UART3, SPI_SLAVE, I2S0, I2S1,
             * I2S2, I2C0, I2C1, I2C2, FPIOA, SHA256, TIMER0,
             * TIMER1, TIMER2
             */
            clock::GPIO => sysctl->clk_en_peri.gpio_clk_en = en;
            clock::SPI2 => sysctl->clk_en_peri.spi2_clk_en = en;
            clock::I2S0 => sysctl->clk_en_peri.i2s0_clk_en = en;
            clock::I2S1 => sysctl->clk_en_peri.i2s1_clk_en = en;
            clock::I2S2 => sysctl->clk_en_peri.i2s2_clk_en = en;
            clock::I2C0 => sysctl->clk_en_peri.i2c0_clk_en = en;
            clock::I2C1 => sysctl->clk_en_peri.i2c1_clk_en = en;
            clock::I2C2 => sysctl->clk_en_peri.i2c2_clk_en = en;
            clock::UART1 => sysctl->clk_en_peri.uart1_clk_en = en;
            clock::UART2 => sysctl->clk_en_peri.uart2_clk_en = en;
            clock::UART3 => sysctl->clk_en_peri.uart3_clk_en = en;
            clock::FPIOA => sysctl->clk_en_peri.fpioa_clk_en = en;
            clock::TIMER0 => sysctl->clk_en_peri.timer0_clk_en = en;
            clock::TIMER1 => sysctl->clk_en_peri.timer1_clk_en = en;
            clock::TIMER2 => sysctl->clk_en_peri.timer2_clk_en = en;
            clock::SHA => sysctl->clk_en_peri.sha_clk_en = en;

            /*
             * These peripheral devices are under APB1
             * WDT, AES, OTP, DVP, SYSCTL
             */
            clock::AES => sysctl->clk_en_peri.aes_clk_en = en;
            clock::WDT0 => sysctl->clk_en_peri.wdt0_clk_en = en;
            clock::WDT1 => sysctl->clk_en_peri.wdt1_clk_en = en;
            clock::OTP => sysctl->clk_en_peri.otp_clk_en = en;
            clock::RTC => sysctl->clk_en_peri.rtc_clk_en = en;
            */
            /*
             * These peripheral devices are under APB2
             * SPI0, SPI1
             */
            clock::SPI0 => (*ptr).clk_en_peri.modify(|_, w| w.spi0_clk_en().bit(en)),
            clock::SPI1 => (*ptr).clk_en_peri.modify(|_, w| w.spi1_clk_en().bit(en)),

            _ => {}
        }
    }
}

pub fn clock_enable(clock: clock) {
    clock_bus_en(clock, true);
    clock_device_en(clock, true);
}

pub fn sysctl_clock_disable(clock: clock) {
    clock_bus_en(clock, false);
    clock_device_en(clock, false);
}

pub fn clock_set_threshold(which: threshold, threshold: u32) {
    unsafe {
        let ptr = pac::SYSCTL::ptr();
        match which {
            /*
            /*
             * These threshold is 2 bit width
             */
            threshold::ACLK =>
                sysctl->clk_sel0.aclk_divider_sel = (uint8_t)threshold & 0x03;

            /*
             * These threshold is 3 bit width
             */
            threshold::APB0 =>
                sysctl->clk_sel0.apb0_clk_sel = (uint8_t)threshold & 0x07;
            threshold::APB1 =>
                sysctl->clk_sel0.apb1_clk_sel = (uint8_t)threshold & 0x07;
            threshold::APB2 =>
                sysctl->clk_sel0.apb2_clk_sel = (uint8_t)threshold & 0x07;

            /*
             * These threshold is 4 bit width
             */
            threshold::SRAM0 =>
                sysctl->clk_th0.sram0_gclk_threshold = (uint8_t)threshold & 0x0F;
            threshold::SRAM1 =>
                sysctl->clk_th0.sram1_gclk_threshold = (uint8_t)threshold & 0x0F;
            threshold::AI =>
                sysctl->clk_th0.ai_gclk_threshold = (uint8_t)threshold & 0x0F;
            threshold::DVP =>
                sysctl->clk_th0.dvp_gclk_threshold = (uint8_t)threshold & 0x0F;
            threshold::ROM =>
                sysctl->clk_th0.rom_gclk_threshold = (uint8_t)threshold & 0x0F;

            /*
             * These threshold is 8 bit width
             */
            */
            threshold::SPI0 => (*ptr)
                .clk_th1
                .modify(|_, w| w.spi0_clk().bits(threshold as u8)),
            /*
            threshold::SPI1 =>
                sysctl->clk_th1.spi1_clk_threshold = (uint8_t)threshold;
            threshold::SPI2 =>
                sysctl->clk_th1.spi2_clk_threshold = (uint8_t)threshold;
            threshold::SPI3 =>
                sysctl->clk_th1.spi3_clk_threshold = (uint8_t)threshold;
            threshold::TIMER0 =>
                sysctl->clk_th2.timer0_clk_threshold = (uint8_t)threshold;
            threshold::TIMER1 =>
                sysctl->clk_th2.timer1_clk_threshold = (uint8_t)threshold;
            threshold::TIMER2 =>
                sysctl->clk_th2.timer2_clk_threshold = (uint8_t)threshold;
            threshold::I2S0_M =>
                sysctl->clk_th4.i2s0_mclk_threshold = (uint8_t)threshold;
            threshold::I2S1_M =>
                sysctl->clk_th4.i2s1_mclk_threshold = (uint8_t)threshold;
            threshold::I2S2_M =>
                sysctl->clk_th5.i2s2_mclk_threshold = (uint8_t)threshold;
            threshold::I2C0 =>
                sysctl->clk_th5.i2c0_clk_threshold = (uint8_t)threshold;
            threshold::I2C1 =>
                sysctl->clk_th5.i2c1_clk_threshold = (uint8_t)threshold;
            threshold::I2C2 =>
                sysctl->clk_th5.i2c2_clk_threshold = (uint8_t)threshold;
            threshold::WDT0 =>
                sysctl->clk_th6.wdt0_clk_threshold = (uint8_t)threshold;
            threshold::WDT1 =>
                sysctl->clk_th6.wdt1_clk_threshold = (uint8_t)threshold;

            /*
             * These threshold is 16 bit width
             */
            threshold::I2S0 =>
                sysctl->clk_th3.i2s0_clk_threshold = (uint16_t)threshold;
            threshold::I2S1 =>
                sysctl->clk_th3.i2s1_clk_threshold = (uint16_t)threshold;
            threshold::I2S2 =>
                sysctl->clk_th4.i2s2_clk_threshold = (uint16_t)threshold;
            */
            _ => {}
        }
    }
}

pub fn clock_get_threshold(which: threshold) -> u32 {
    unsafe {
        let ptr = pac::SYSCTL::ptr();
        match which {
            threshold::ACLK => (*ptr).clk_sel0.read().aclk_divider_sel().bits().into(),
            threshold::SPI0 => (*ptr).clk_th1.read().spi0_clk().bits().into(),

            _ => {
                panic!("no such threshold");
            }
        }
    }
}

pub fn set_power_mode(power_bank: power_bank, mode: io_power_mode) {
    let power_bank = power_bank as u32;
    unsafe {
        (*pac::SYSCTL::ptr()).power_sel.modify(|r, w| {
            w.bits(set_bit(
                r.bits(),
                power_bank as u8,
                match mode {
                    io_power_mode::V33 => false,
                    io_power_mode::V18 => true,
                },
            ))
        });
    }
}

pub fn set_spi0_dvp_data(status: bool) {
    unsafe {
        (*pac::SYSCTL::ptr())
            .misc
            .modify(|_, w| w.spi_dvp_data_enable().bit(status));
    }
}

pub fn pll_get_freq(pll: pll) -> u32 {
    let freq_in;
    let nr;
    let nf;
    let od;

    match pll {
        pll::PLL0 => {
            freq_in = clock_source_get_freq(clock_source::IN0);
            unsafe {
                let val = (*pac::SYSCTL::ptr()).pll0.read();
                nr = val.clkr().bits() + 1;
                nf = val.clkf().bits() + 1;
                od = val.clkod().bits() + 1;
            }
        }

        pll::PLL1 => {
            freq_in = clock_source_get_freq(clock_source::IN0);
            unsafe {
                let val = (*pac::SYSCTL::ptr()).pll1.read();
                nr = val.clkr().bits() + 1;
                nf = val.clkf().bits() + 1;
                od = val.clkod().bits() + 1;
            }
        }
        pll::PLL2 => {
            /* Get input freq accrording to select register. */
            freq_in = clock_source_get_freq(match clock_get_clock_select(clock_select::PLL2) {
                0 => clock_source::IN0,
                1 => clock_source::PLL0,
                2 => clock_source::PLL1,
                _ => panic!("unknown PLL2 source"),
            });
            unsafe {
                let val = (*pac::SYSCTL::ptr()).pll2.read();
                nr = val.clkr().bits() + 1;
                nf = val.clkf().bits() + 1;
                od = val.clkod().bits() + 1;
            }
        }
    }

    /*
     * Get final PLL output freq
     * FOUT = FIN / NR * NF / OD
     * (rewritten as integer expression)
     */
    (((freq_in as u64) * (nf as u64)) / ((nr as u64) * (od as u64))) as u32
}

pub fn clock_source_get_freq(source: clock_source) -> u32 {
    match source {
        clock_source::IN0 => SYSCTRL_CLOCK_FREQ_IN0,
        clock_source::PLL0 => pll_get_freq(pll::PLL0),
        clock_source::PLL1 => pll_get_freq(pll::PLL1),
        clock_source::PLL2 => pll_get_freq(pll::PLL2),
        clock_source::ACLK => clock_get_freq(clock::ACLK),
    }
}

pub fn clock_get_clock_select(which: clock_select) -> u8 {
    unsafe {
        let ptr = pac::SYSCTL::ptr();
        match which {
            /*
             * Select and get clock select value
             */
            clock_select::PLL0_BYPASS => (*ptr).pll0.read().bypass().bit().into(),
            clock_select::PLL1_BYPASS => (*ptr).pll1.read().bypass().bit().into(),
            clock_select::PLL2_BYPASS => (*ptr).pll2.read().bypass().bit().into(),
            clock_select::PLL2 => (*ptr).pll2.read().ckin_sel().bits().into(),
            clock_select::ACLK => (*ptr).clk_sel0.read().aclk_sel().bit().into(),
            clock_select::SPI3 => (*ptr).clk_sel0.read().spi3_clk_sel().bit().into(),
            clock_select::TIMER0 => (*ptr).clk_sel0.read().timer0_clk_sel().bit().into(),
            clock_select::TIMER1 => (*ptr).clk_sel0.read().timer1_clk_sel().bit().into(),
            clock_select::TIMER2 => (*ptr).clk_sel0.read().timer2_clk_sel().bit().into(),
            clock_select::SPI3_SAMPLE => (*ptr).clk_sel1.read().spi3_sample_clk_sel().bit().into(),
        }
    }
}

pub fn clock_get_freq(clock: clock) -> u32 {
    match clock {
        clock::PLL0 => clock_source_get_freq(clock_source::PLL0),
        clock::PLL1 => clock_source_get_freq(clock_source::PLL1),
        clock::PLL2 => clock_source_get_freq(clock_source::PLL2),
        clock::CPU => match clock_get_clock_select(clock_select::ACLK) {
            0 => clock_source_get_freq(clock_source::IN0),
            1 => {
                clock_source_get_freq(clock_source::PLL0)
                    / (2 << clock_get_threshold(threshold::ACLK))
            }
            _ => panic!("invalid cpu clock select"),
        },
        clock::SPI0 => {
            let source = clock_source_get_freq(clock_source::PLL0);
            source / ((clock_get_threshold(threshold::SPI0) + 1) * 2)
        }
        _ => panic!("not implemented"),
    }
}
