Kendryte K210 memory map
------------------------

This is a rough memory map for the Kendryte K210 (as used on the Sipeed Maix boards). For some reason this is missing from the [data sheet](https://s3.cn-north-1.amazonaws.com.cn/dl.kendryte.com/documents/kendryte_datasheet_20181011163248_en.pdf). I've tried to use the same naming when possible.

    from       to           description
    ---------- ----------   ---------------------------------------
    0x00000000 0x0fffffff   CorePlex
    0x00000000 0x00000fff     DEBUG
    0x00001000 0x00001fff     ROMCPU
    0x02000000 0x03ffffff     CLINT
    0x0c000000 0x0fffffff     PLIC
    0x38000000 0x3fffffff   TileLink
    0x38000000                UARTHS
    0x38001000                GPIOHS
    0x40000000 0x4fffffff   AXI 64-bit (non-cached)
    0x40000000 0x403fffff     General-purpose SRAM MEM0 (non-cached)
    0x40400000 0x405fffff     General-purpose SRAM MEM1 (non-cached)
    0x40600000 0x407fffff     AI SRAM (non-cached)
    0x40800000                KPU
    0x42000000 0x423fffff     FFT
    0x50000000 0x501fffff   AHB 32-bit
    0x50000000                DMAC
    0x50200000 0x503fffff   APB1 32 bit
    0x50200000                GPIO
    0x50210000                UART1
    0x50220000                UART2
    0x50230000                UART3
    0x50240000                SPI2
    0x50250000                I2S0
    0x50250200                APU
    0x50260000                I2S1
    0x50270000                I2S2
    0x50280000                I2C0
    0x50290000                I2C1
    0x502a0000                I2C2
    0x502b0000                FPIOA
    0x502c0000                SHA256
    0x502d0000                TIMER0
    0x502e0000                TIMER1
    0x502f0000                TIMER2
    0x50400000 0x505fffff   APB2 32 bit
    0x50400000                WDT0
    0x50410000                WDT1
    0x50420000                OTP
    0x50430000                DVP
    0x50440000                SYSCTL
    0x50450000                AES
    0x50460000                RTC
    0x52000000 0x5???????   APB3 32 bit
    0x52000000                SPI0
    0x53000000                SPI1
    0x54000000                SPI3
    0x80000000 0x8fffffff   AXI 64-bit (cached mirror of 0x40000000)
    0x80000000 0x803fffff     General-purpose SRAM MEM0 (cached)
    0x80400000 0x805fffff     General-purpose SRAM MEM1 (cached)
    0x80600000 0x807fffff     AI SRAM (cached)
    0x88000000 0x8801ffff     ROM

Source (paths refer to Kendryte stand-alone SDK):
- `lib/bsp/include/platform.h`
- `lib/drivers/apu.c`
- ROM dump (there's a compiled device tree in there)

RAM ranges
-----------

- The 2MB of AI SRAM at `0x80600000` can be used as normal RAM when the KPU is not used.

- The `0x4xxxxxxx` range has a non-cached mirror of the SRAMs. Interestingly, floating point load instructions [don't work](https://github.com/rust-embedded/riscv-rt/issues/25#issuecomment-491524341) from this area.

- There is a mirror of the cached SRAM area at `0xffffffff80000000`, this is useful for the [medlow memory model](https://www.sifive.com/blog/all-aboard-part-4-risc-v-code-models).

  - More generally, it looks like the upper 32 bits of the address are ignored.

Boot sequence
-------------

`ROMCPU` (`0x00001000`) seems to be the initial boot vector, from there, there's a jump to the beginning of the main ROM (`0x88000000`). This ROM implements loading of the flash to memory at `0x80000000` and jumping to it, as well as "ISP" mode for directly uploading a program from the serial port (this is used by `kflash.py` for flashing).

### ISP mode

From the datasheet:

> IO_16 is used for boot mode selection. During power-on reset, pull high to bootfrom FLASH and pull low to enter ISP mode. After reset, IO_0, IO_1, IO_2, and IO_3 are JTAG pins. IO_4 and IO_5 are ISP pins

A packet-based protocol over serial (at baudrate 115200) is used to communicate with the device. The following commands are documented to exist in ISP mode (source: `kflash.py`):

    0xC1 ECHO (not implemented, returns INVALID_COMMAND)
    0xC2 NOP
    0xC3 MEMORY_WRITE
    0xC4 MEMORY_READ (not implemented, returns INVALID_COMMAND)
    0xC5 MEMORY_BOOT
    0xC6 UARTHS_BAUDRATE_SET

Packets are encoded in so-called SLIP format. This means `0xc0` is used as packet beginning and end marker, the escape sequence `0xdb 0xdc` is used to encode `0xc0`, and `0xdb 0xdd` encodes `0xdb`. The first byte of a packet is the command or response code.

The following responses can be returned:

    0xE0 OK
    0xE1 BAD_DATA_LEN
    0xE2 BAD_DATA_CHECKSUM
    0xE3 INVALID_COMMAND

- Setting the baud rate in the ISP to anything besides 115200 with `UARTHS_BAUDRATE_SET` doesn't work reliably.
  According to Kendryte trying to do so is [dinosaur-infested](https://github.com/kendryte/kflash.py/blob/master/kflash.py#L513),
  which means the higher baudrate used there (1500000) only works with some boards.

IO maps for devices
===================

Registers are 32-bit unless mentioned otherwise.

AES
---

(source: `lib/drivers/include/aes.h`)

| Ofs   | Name              | Description                                                   |
| ----- | ----------------- | ------------------------------------------------------------- |
| 0x00  | `aes_key[4]`      | customer key.1st~4th byte key                                 |
| 0x10  | `encrypt_sel`     | 0: encryption; 1: decryption                                  |
| 0x14  | `mode_ctl`        | aes mode reg                                                  |
| 0x18  | `aes_iv[4]`       | Initialisation Vector. GCM support 96bit. CBC support 128bit  |
| 0x28  | `aes_endian`      | input data endian;1:little endian; 0:big endian               |
| 0x2c  | `aes_finish`      | calculate status. 1:finish; 0:not finish                      |
| 0x30  | `dma_sel`         | aes out data to dma 0:cpu 1:dma                               |
| 0x34  | `gb_aad_num`      | gcm Additional authenticated data number                      |
| 0x38  | reserved          |                                                               |
| 0x3c  | `gb_pc_num`       | aes plantext/ciphter text input data number                   |
| 0x40  | `aes_text_data`   | aes plantext/ciphter text input data                          |
| 0x44  | `aes_aad_data`    | Additional authenticated data                                 |
| 0x48  | `tag_chk`         | [1:0],b'00:check not finish; b'01:check fail; b'10:check success; b'11:reversed |
| 0x4c  | `data_in_flag`    | data can input flag. 1: data can input; 0 : data cannot input |
| 0x50  | `gcm_in_tag[4]`   | gcm input tag for compare with the calculate tag              |
| 0x60  | `aes_out_data`    | aes plantext/ciphter text output data                         |
| 0x64  | `gb_aes_en`       | aes module enable                                             |
| 0x68  | `data_out_flag`   | data can output flag 1: data ready 0: data not ready          |
| 0x6c  | `tag_in_flag`     | allow tag input when use gcm                                  |
| 0x70  | `tag_clear`       | clear `tag_chk`                                               |
| 0x74  | `gcm_out_tag[4]`  | gcm tag output                                                |
| 0x84  | `aes_key_ext[4]`  | customer key for aes-192 aes-256.5th~8th byte key             |

APU
---

(source: `lib/drivers/include/apu.h`)

| Ofs   | Name                   | Description                                                   |
| ----- | ---------------------- | ------------------------------------------------------------- |
| 0x000 | `bf_ch_cfg_reg`        | Channel Config Register                                       |
| 0x004 | `bf_ctl_reg`           | Control Register                                              |
| 0x008 | `bf_dir_bidx[16][2]`   | Direction Sample Buffer Read Index Configure Register         |
| 0x088 | `bf_pre_fir0_coef[9]`  | FIR0 pre-filter coefficients                                  |
| 0x0ac | `bf_post_fir0_coef[9]` | FIR0 post-filter coefficients                                 |
| 0x0d0 | `bf_pre_fir1_coef[9]`  | FIR1 pre-filter coeffecients                                  |
| 0x0f4 | `bf_post_fir1_coef[9]` | FIR1 post-filter coefficients                                 |
| 0x118 | `bf_dwsz_cfg_reg`      | Downsize Config Register                                      |
| 0x11c | `bf_fft_cfg_reg`       | FFT Config Register                                           |
| 0x120 | `sobuf_dma_rdata`      | Read register for DMA to sample-out buffers                   |
| 0x124 | `vobuf_dma_rdata`      | Read register for DMA to voice-out buffers                    |
| 0x128 | `bf_int_stat_reg`      | Interrupt Status Register                                     |
| 0x12c | `bf_int_mask_reg`      | Interrupt Mask Register                                       |
| 0x130 | `saturation_counter`   | Saturation Counter                                            |
| 0x134 | `saturation_limits`    | Saturation Limits                                             |

CLINT
-----

(source: `lib/drivers/include/clint.h`, addresses are absolute)

| Address    | Description                     |
|------------|---------------------------------|
| 0x02000000 | `msip` for core 0               |
| 0x02000004 | `msip` for core 1               |
| ...        | ...                             |
| 0x02003ff8 | `msip` for core 4094            |
|            |                                 |
| 0x02004000 | `mtimecmp` for core 0           |
| 0x02004008 | `mtimecmp` for core 1           |
| ...        | ...                             |
| 0x0200bff0 | `mtimecmp` For core 4094        |
| 0x0200bff8 | `mtime`                         |
|            |                                 |
| 0x0200c000 | Reserved                        |
| ...        | ...                             |
| 0x0200effc | Reserved                        |

This looks similar to, or even the same as, the SiFive CLINT.

DEBUG
-----

Likely follows the [RISC-V debug spec](https://github.com/riscv/riscv-debug-spec/blob/release/riscv-debug-release.pdf)
and is only accessible from debug mode through JTAG.

DMAC
----

(source: `lib/drivers/include/dmac.h`)

Registers are 64 bit.

| Ofs   | Name              | Description                                                   |
| ----- | ----------------- | ------------------------------------------------------------- |
| 0x000 | `id`              | DMAC ID Rgister                                               |
| 0x008 | `compver`         | DMAC COMPVER Register                                         |
| 0x010 | `cfg`             | DMAC Configure Register                                       |
| 0x018 | `chen`            | Channel Enable Register                                       |
| 0x020 | `reserved[2]`     |                                                               |
| 0x030 | `intstatus`       | DMAC Interrupt Status Register                                |
| 0x038 | `com_intclear`    | DMAC Common register Interrupt Status Register                |
| 0x040 | `com_intstatus_en` | DMAC Common Interrupt Enable Register                        |
| 0x048 | `com_intsignal_en` | DMAC Common Interrupt Signal Enable Register                 |
| 0x050 | `com_intstatus`   | DMAC Common Interrupt Status                                  |
| 0x058 | `reset`           | DMAC Reset register                                           |
| 0x060 | `reserved[20]`    |                                                               |
| 0x100 | `channel[6]`      | DMA channels configuration                                    |

DVP
---

(source: `lib/drivers/include/dvp.h`)

| Ofs   | Name              | Description                         |
| ----- | ----------------- | ----------------------------------- |
| 0x00  | `dvp_cfg`         | Config Register                     |
| 0x04  | `r_addr`          | Output address for red component    |
| 0x08  | `g_addr`          | Output address for green component  |
| 0x0c  | `b_addr`          | Output address for blue component   |
| 0x10  | `cmos_cfg`        | CMOS Config Register                |
| 0x14  | `sccb_cfg`        | SCCB Config Register                |
| 0x18  | `sccb_ctl`        | SCCB Control Register               |
| 0x1c  | `axi`             | AXI Register                        |
| 0x20  | `sts`             | STS Register                        |
| 0x24  | `reverse`         |                                     |
| 0x28  | `rgb_addr`        | Output address for R5G6B5 data      |

FFT
---

(source: `lib/drivers/include/fft.h`)

| Ofs   | Name              | Description                         |
| ----- | ----------------- | ----------------------------------- |
| 0x00  | `fft_input_fifo`  | Input data FIFO                     |
| 0x08  | `fft_ctrl`        | FFT ctrl reg                        |
| 0x10  | `fifo_ctrl`       | FIFO ctrl                           |
| 0x18  | `intr_mask`       | Interrupt mask                      |
| 0x20  | `intr_clear`      | Interrupt clear                     |
| 0x28  | `fft_status`      | FFT status reg                      |
| 0x30  | `fft_status_raw`  | FFT status raw                      |
| 0x38  | `fft_output_fifo` | FFT output FIFO                     |

FPIOA
-----

(source: `lib/drivers/include/fpioa.h`)

| Ofs   | Name              | Description                                                   |
| ----- | ----------------- | ------------------------------------------------------------- |
| 0x00  | `io[48]`          | FPIOA GPIO multiplexer io array                               |
| 0xc0  | `tie_en[8]`       | FPIOA GPIO multiplexer tie enable                             |
| 0xe0  | `tie_val[8]`      | FPIOA GPIO multiplexer tie value                              |

GPIO
----

(source: `lib/drivers/include/gpio.h`)

| Ofs   | Name              | Description                                                   |
| ----- | ----------------- | ------------------------------------------------------------- |
| 0x00  | `data_output`      | Data (output) registers                                      |
| 0x04  | `direction`        | Data direction registers                                     |
| 0x08  | `source`           | Data source registers                                        |
| 0x10  | `unused_0[9]`      | Unused registers, 9x4 bytes                                  |
| 0x30  | `interrupt_enable` | Interrupt enable/disable registers                           |
| 0x34  | `interrupt_mask`   | Interrupt mask registers                                     |
| 0x38  | `interrupt_level`  | Interrupt level registers                                    |
| 0x3c  | `interrupt_polarity` | Interrupt polarity registers                               |
| 0x40  | `interrupt_status` | Interrupt status registers                                   |
| 0x44  | `interrupt_status_raw` | Raw interrupt status registers                           |
| 0x48  | `interrupt_debounce` | Interrupt debounce registers                               |
| 0x4c  | `interrupt_clear`  | Registers for clearing interrupts                            |
| 0x50  | `data_input`       | External port (data input) registers                         |
| 0x54  | `unused_1[3]`      | Unused registers, 3x4 bytes                                  |
| 0x60  | `sync_level`       | Sync level registers                                         |
| 0x64  | `id_code`          | ID code                                                      |
| 0x68  | `interrupt_bothedge` | Interrupt both edge type                                   |

GPIOHS
------

(source: `lib/drivers/include/gpiohs.h`)

| Ofs   | Name              | Description                                                   |
| ----- | ----------------- | ------------------------------------------------------------- |
| 0x00  | `input_val`       |                                                               |
| 0x04  | `input_en`        |                                                               |
| 0x08  | `output_en`       |                                                               |
| 0x0c  | `output_val`      |                                                               |
| 0x10  | `pullup_en`       |                                                               |
| 0x14  | `drive`           |                                                               |
| 0x18  | `rise_ie`         |                                                               |
| 0x1c  | `rise_ip`         |                                                               |
| 0x20  | `fall_ie`         |                                                               |
| 0x24  | `fall_ip`         |                                                               |
| 0x28  | `high_ie`         |                                                               |
| 0x2c  | `high_ip`         |                                                               |
| 0x30  | `low_ie`          |                                                               |
| 0x34  | `low_ip`          |                                                               |
| 0x38  | `iof_en`          |                                                               |
| 0x3c  | `iof_sel`         |                                                               |
| 0x40  | `output_xor`      |                                                               |

I2Cx
----

(source: `lib/drivers/include/i2c.h`)

| Ofs   | Name              | Description                                                   |
| ----- | ----------------- | ------------------------------------------------------------- |
| 0x00  | `con`             | I2C Control Register                                          |
| 0x04  | `tar`             | I2C Target Address Register                                   |
| 0x08  | `sar`             | I2C Slave Address Register                                    |
| 0x0c  | `resv1`           | reserved                                                      |
| 0x10  | `data_cmd`        | I2C Data Buffer and Command Register                          |
| 0x14  | `ss_scl_hcnt`     | I2C Standard Speed Clock SCL High Count Register              |
| 0x18  | `ss_scl_lcnt`     | I2C Standard Speed Clock SCL Low Count Register               |
| 0x1c  | `resv2[4]`        | reserverd                                                     |
| 0x2c  | `intr_stat`       | I2C Interrupt Status Register                                 |
| 0x30  | `intr_mask`       | I2C Interrupt Mask Register                                   |
| 0x34  | `raw_intr_stat`   | I2C Raw Interrupt Status Register                             |
| 0x38  | `rx_tl`           | I2C Receive FIFO Threshold Register                           |
| 0x3c  | `tx_tl`           | I2C Transmit FIFO Threshold Register                          |
| 0x40  | `clr_intr`        | I2C Clear Combined and Individual Interrupt Register          |
| 0x44  | `clr_rx_under`    | I2C Clear RX\_UNDER Interrupt Register                        |
| 0x48  | `clr_rx_over`     | I2C Clear RX\_OVER Interrupt Register                         |
| 0x4c  | `clr_tx_over`     | I2C Clear TX\_OVER Interrupt Register                         |
| 0x50  | `clr_rd_req`      | I2C Clear RD\_REQ Interrupt Register                          |
| 0x54  | `clr_tx_abrt`     | I2C Clear TX\_ABRT Interrupt Register                         |
| 0x58  | `clr_rx_done`     | I2C Clear RX\_DONE Interrupt Register                         |
| 0x5c  | `clr_activity`    | I2C Clear ACTIVITY Interrupt Register                         |
| 0x60  | `clr_stop_det`    | I2C Clear STOP\_DET Interrupt Register                        |
| 0x64  | `clr_start_det`   | I2C Clear START\_DET Interrupt Register                       |
| 0x68  | `clr_gen_call`    | I2C Clear GEN\_CALL Interrupt Register                        |
| 0x6c  | `enable`          | I2C Enable Register                                           |
| 0x70  | `status`          | I2C Status Register                                           |
| 0x74  | `txflr`           | I2C Transmit FIFO Level Register                              |
| 0x78  | `rxflr`           | I2C Receive FIFO Level Register                               |
| 0x7c  | `sda_hold`        | I2C SDA Hold Time Length Register                             |
| 0x80  | `tx_abrt_source`  | I2C Transmit Abort Source Register                            |
| 0x84  | `resv3`           | reserved                                                      |
| 0x88  | `dma_cr`          | I2C DMA Control Register                                      |
| 0x8c  | `dma_tdlr`        | I2C DMA Transmit Data Level Register                          |
| 0x90  | `dma_rdlr`        | I2C DMA Receive Data Level Register                           |
| 0x94  | `sda_setup`       | I2C SDA Setup Register                                        |
| 0x98  | `general_call`    | I2C ACK General Call Register                                 |
| 0x9c  | `enable_status`   | I2C Enable Status Register                                    |
| 0xa0  | `fs_spklen`       | I2C SS, FS or FM+ spike suppression limit                     |
| 0xa4  | `resv4[20]`       | reserved                                                      |
| 0xf4  | `comp_param_1`    | I2C Component Parameter Register 1                            |
| 0xf8  | `comp_version`    | I2C Component Version Register                                |
| 0xfc  | `comp_type`       | I2C Component Type Register                                   |

This block is replicated for all 3 I2C peripherals.

I2Sx
----

(source: `lib/drivers/include/i2s.h`)

| Ofs   | Name              | Description                                                   |
| ----- | ----------------- | ------------------------------------------------------------- |
| 0x000 | `ier`             | I2S Enable Register                                           |
| 0x004 | `irer`            | I2S Receiver Block Enable Register                            |
| 0x008 | `iter`            | I2S Transmitter Block Enable Register                         |
| 0x00c | `cer`             | Clock Enable Register                                         |
| 0x010 | `ccr`             | Clock Configuration Register                                  |
| 0x014 | `rxffr`           | Receiver Block FIFO Reset Register                            |
| 0x018 | `txffr`           | Transmitter Block FIFO Reset Register                         |
| 0x01c | `reserved1`       |                                                               |
| 0x020 | `channel[4]`      | Channel setup (64 bytes per channel)                          |
| 0x120 | `reserved2[40]`   |                                                               |
| 0x1c0 | `rxdma`           | Receiver Block DMA Register                                   |
| 0x1c4 | `rrxdma`          | Reset Receiver Block DMA Register                             |
| 0x1c8 | `txdma`           | Transmitter Block DMA Register                                |
| 0x1cc | `rtxdma`          | Reset Transmitter Block DMA Register                          |
| 0x1d0 | `reserved3[8]`    | reserved                                                      |
| 0x1f0 | `i2s_comp_param_2` | Component Parameter Register 2                               |
| 0x1f4 | `i2s_comp_param_1` | Component Parameter Register 1                               |
| 0x1f8 | `i2s_comp_version_1`| I2S Component Version Register                              |
| 0x1fc | `i2s_comp_type`   | I2S Component Type Register                                   |

This block is replicated for all 3 I2S peripherals.

KPU
---

(source: `lib/drivers/include/kpu.h`)

Registers are 64-bit.

| Ofs   | Name              | Description                                                   |
| ----- | ----------------- | ------------------------------------------------------------- |
| 0x00  | `layer_argument_fifo` |                                                           |
| 0x08  | `interrupt_status` |                                                              |
| 0x10  | `interrupt_raw`   |                                                               |
| 0x18  | `interrupt_mask`  |                                                               |
| 0x20  | `interrupt_clear` |                                                               |
| 0x28  | `fifo_threshold`  |                                                               |
| 0x30  | `fifo_data_out`   |                                                               |
| 0x38  | `fifo_ctrl`       |                                                               |
| 0x40  | `eight_bit_mode`  | Enable 8-bit instead of 16-bit precision                      |

OTP
---

(seems undocumented, and only used from ROM)

```
0x68   104  r      fuses A?
0xb8   184  r      fuses B1?
0xbc   188  r      fuses B0?
```

The fuse bits determine some choices during the ROM boot process, such as from what flash to boot,
whether to decrypt flash before boot, and so on.

There is also a 128 kbit one-time programmable memory that can be read (and
written) through this interface, and it can be prompted to provide a AES128 key to
the AES peripheral without exposing it to software.

PLIC
----

(source: `lib/drivers/include/plic.h`, addresses are absolute)

| Address    | Description                     |
| ---------- | ------------------------------- |
| 0x0c000000 | Reserved                        |
| 0x0c000004 | source 1 priority               |
| 0x0c000008 | source 2 priority               |
| ...        | ...                             |
| 0x0c000ffC | source 1023 priority            |
|            |                                 |
| 0x0c001000 | Start of pending array          |
| ...        | (read-only)                     |
| 0x0c00107c | End of pending array            |
| 0x0c001080 | Reserved                        |
| ...        | ...                             |
| 0x0c001fff | Reserved                        |
|            |                                 |
| 0x0c002000 | target 0 enables                |
| 0x0c002080 | target 1 enables                |
| ...        | ...                             |
| 0x0c1f1f80 | target 15871 enables            |
| 0x0c1f2000 | Reserved                        |
| ...        | ...                             |
| 0x0c1ffffc | Reserved                        |
|            |                                 |
| 0x0c200000 | target 0 priority threshold     |
| 0x0c200004 | target 0 claim/complete         |
| ...        | ...                             |
| 0x0c201000 | target 1 priority threshold     |
| 0x0c201004 | target 1 claim/complete         |
| ...        | ...                             |
| 0x0ffff000 | target 15871 priority threshold |
| 0x0ffff004 | target 15871 claim/complete     |

This looks similar to, or even the same as, the SiFive PLIC.

RTC
---

(source: `lib/drivers/include/rtc.h`)

| Ofs   | Name              | Description                                                   |
| ----- | ----------------- | ------------------------------------------------------------- |
| 0x00  | `date`            | Timer date information                                        |
| 0x04  | `time`            | Timer time information                                        |
| 0x08  | `alarm_date`      | Alarm date information                                        |
| 0x0c  | `alarm_time`      | Alarm time information                                        |
| 0x10  | `initial_count`   | Timer counter initial value                                   |
| 0x14  | `current_count`   | Timer counter current value                                   |
| 0x18  | `interrupt_ctrl`  | RTC interrupt settings                                        |
| 0x1c  | `register_ctrl`   | RTC register settings                                         |
| 0x20  | `reserved0`       | Reserved                                                      |
| 0x24  | `reserved1`       | Reserved                                                      |
| 0x28  | `extended`        | Timer extended information                                    |

SHA256
------

(source: `lib/drivers/include/sha256.h`)

| Ofs   | Name              | Description                                                   |
| ----- | ----------------- | ------------------------------------------------------------- |
| 0x00  | `sha_result[8]`   | Calculated SHA256 return value                                |
| 0x20  | `sha_data_in1`    | SHA256 input data from this register                          |
| 0x24  | `reserved0`       |                                                               |
| 0x28  | `sha_num_reg`     |                                                               |
| 0x2c  | `sha_function_reg_0` |                                                            |
| 0x30  | `reserved1`       |                                                               |
| 0x34  | `sha_function_reg_1` |                                                            |

SPIx
----

(source: `lib/drivers/include/spi.h`)

| Ofs   | Name              | Description                                                   |
| ----- | ----------------- | ------------------------------------------------------------- |
| 0x000 | `ctrlr0`          | SPI Control Register 0                                        |
| 0x004 | `ctrlr1`          | SPI Control Register 1                                        |
| 0x008 | `ssienr`          | SPI Enable Register                                           |
| 0x00c | `mwcr`            | SPI Microwire Control Register                                |
| 0x010 | `ser`             | SPI Slave Enable Register                                     |
| 0x014 | `baudr`           | SPI Baud Rate Select                                          |
| 0x018 | `txftlr`          | SPI Transmit FIFO Threshold Level                             |
| 0x01c | `rxftlr`          | SPI Receive FIFO Threshold Level                              |
| 0x020 | `txflr`           | SPI Transmit FIFO Level Register                              |
| 0x024 | `rxflr`           | SPI Receive FIFO Level Register                               |
| 0x028 | `sr`              | SPI Status Register                                           |
| 0x02c | `imr`             | SPI Interrupt Mask Register                                   |
| 0x030 | `isr`             | SPI Interrupt Status Register                                 |
| 0x034 | `risr`            | SPI Raw Interrupt Status Register                             |
| 0x038 | `txoicr`          | SPI Transmit FIFO Overflow Interrupt Clear Register           |
| 0x03c | `rxoicr`          | SPI Receive FIFO Overflow Interrupt Clear Register            |
| 0x040 | `rxuicr`          | SPI Receive FIFO Underflow Interrupt Clear Register           |
| 0x044 | `msticr`          | SPI Multi-Master Interrupt Clear Register                     |
| 0x048 | `icr`             | SPI Interrupt Clear Register                                  |
| 0x04c | `dmacr`           | SPI DMA Control Register                                      |
| 0x050 | `dmatdlr`         | SPI DMA Transmit Data Level                                   |
| 0x054 | `dmardlr`         | SPI DMA Receive Data Level                                    |
| 0x058 | `idr`             | SPI Identification Register                                   |
| 0x05c | `ssic_version_id` | SPI DWC\_ssi component version                                |
| 0x060 | `dr[36]`          | SPI Data Register 0-36                                        |
| 0x0f0 | `rx_sample_delay` | SPI RX Sample Delay Register                                  |
| 0x0f4 | `spi_ctrlr0`      | SPI SPI Control Register                                      |
| 0x0f8 | `resv`            |                                                               |
| 0x0fc | `xip_mode_bits`   | SPI XIP Mode bits                                             |
| 0x100 | `xip_incr_inst`   | SPI XIP INCR transfer opcode                                  |
| 0x104 | `xip_wrap_inst`   | SPI XIP WRAP transfer opcode                                  |
| 0x108 | `xip_ctrl`        | SPI XIP Control Register                                      |
| 0x10c | `xip_ser`         | SPI XIP Slave Enable Register                                 |
| 0x110 | `xrxoicr`         | SPI XIP Receive FIFO Overflow Interrupt Clear Register        |
| 0x114 | `xip_cnt_time_out` | SPI XIP time out register for continuous transfers           |
| 0x118 | `endian`          | SPI Endian                                                    |

This block is replicated for all 4 SPI peripherals except for SPI2, which is SPI
slave-only and has a slightly different interface.

SYSCTL
------

(source: `lib/drivers/include/sysctl.h`)

| Ofs   | Name           | Description                         |
| ----- | -------------- | ----------------------------------- |
| 0x00  | `git_id`       | Git short commit id                 |
| 0x04  | `clk_freq`     | System clock base frequency         |
| 0x08  | `pll0`         | PLL0 controller                     |
| 0x0c  | `pll1`         | PLL1 controller                     |
| 0x10  | `pll2`         | PLL2 controller                     |
| 0x14  | `resv5`        | Reserved                            |
| 0x18  | `pll_lock`     | PLL lock tester                     |
| 0x1c  | `rom_error`    | AXI ROM detector                    |
| 0x20  | `clk_sel0`     | Clock select controller0            |
| 0x24  | `clk_sel1`     | Clock select controller1            |
| 0x28  | `clk_en_cent`  | Central clock enable                |
| 0x2c  | `clk_en_peri`  | Peripheral clock enable             |
| 0x30  | `soft_reset`   | Soft reset ctrl                     |
| 0x34  | `peri_reset`   | Peripheral reset controller         |
| 0x38  | `clk_th0`      | Clock threshold controller 0        |
| 0x3c  | `clk_th1`      | Clock threshold controller 1        |
| 0x40  | `clk_th2`      | Clock threshold controller 2        |
| 0x44  | `clk_th3`      | Clock threshold controller 3        |
| 0x48  | `clk_th4`      | Clock threshold controller 4        |
| 0x4c  | `clk_th5`      | Clock threshold controller 5        |
| 0x50  | `clk_th6`      | Clock threshold controller 6        |
| 0x54  | `misc`         | Miscellaneous controller            |
| 0x58  | `peri`         | Peripheral controller               |
| 0x5c  | `spi_sleep`    | SPI sleep controller                |
| 0x60  | `reset_status` | Reset source status                 |
| 0x64  | `dma_sel0`     | DMA handshake selector              |
| 0x68  | `dma_sel1`     | DMA handshake selector              |
| 0x6c  | `power_sel`    | IO Power Mode Select controller     |
| 0x70  | `resv28`       | Reserved                            |
| 0x74  | `resv29`       | Reserved                            |
| 0x78  | `resv30`       | Reserved                            |
| 0x7c  | `resv31`       | Reserved                            |

TIMERx
------

(source: `lib/drivers/include/timer.h`)

| Ofs   | Name              | Description                                                   |
| ----- | ----------------- | ------------------------------------------------------------- |
| 0x00  | `channel[4]`      | TIMER\_N Register (20 bytes per channel)                      |
| 0x50  | `resv1[20]`       |                                                               |
| 0xa0  | `intr_stat`       | TIMER Interrupt Status Register                               |
| 0xa4  | `eoi`             | TIMER Interrupt Clear Register                                |
| 0xa8  | `raw_intr_stat`   | TIMER Raw Interrupt Status Register                           |
| 0xac  | `comp_version`    | TIMER Component Version Register                              |
| 0xb0  | `load_count2[4]`  | TIMER\_N Load Count2 Register                                 |

UARTx
-----

(source: `lib/drivers/include/uart.h`)

| Ofs   | Name              | Description                                                   |
| ----- | ----------------- | ------------------------------------------------------------- |
| 0x00  | `RBR`             | Receive Buffer Register - lower 8 bits (read)                 |
| 0x00  | `DLL`             | Divisor Latch LSB                                             |
| 0x00  | `THR`             | Transmitter Holding Register - lower 8 bits (write)           |
| 0x04  | `DLH`             | Divisor latch MSB                                             |
| 0x04  | `IER`             | Interrupt Enable Register                                     |
| 0x08  | `FCR`             | FIFO Control Register                                         |
| 0x08  | `IIR`             | Interrupt Identification Register                             |
| 0x0c  | `LCR`             | Line Control Register                                         |
| 0x10  | `MCR`             | Modem Control Register                                        |
| 0x14  | `LSR`             | Line Status Register                                          |
| 0x18  | `MSR`             | Modem Status Register                                         |
| 0x1c  | `SCR`             | Scratch                                                       |
| 0x20  | `LPDLL`           | Low Power Divisor Latch (Low) Register                        |
| 0x24  | `LPDLH`           | Low Power Divisor Latch (High) Register                       |
| 0x28  | `reserved1[2]`    |                                                               |
| 0x30  | `SRBR[16]`        | Shadow Receive Buffer Register                                |
| 0x30  | `STHR[16]`        | Shadow Transmit Holding Register                              |
| 0x70  | `FAR`             | FIFO Access Register                                          |
| 0x74  | `TFR`             | Transmit FIFO Read Register                                   |
| 0x78  | `RFW`             | Receive FIFO Write Register                                   |
| 0x7c  | `USR`             | UART Status Register                                          |
| 0x80  | `TFL`             | Transmit FIFO Level                                           |
| 0x84  | `RFL`             | Receive FIFO Level                                            |
| 0x88  | `SRR`             | Software Reset Register                                       |
| 0x8c  | `SRTS`            | Shadow Request to Send Register                               |
| 0x90  | `SBCR`            | Shadow Break Control Register                                 |
| 0x94  | `SDMAM`           | Shadow DMA Mode                                               |
| 0x98  | `SFE`             | Shadow FIFO Enable                                            |
| 0x9c  | `SRT`             | Shadow RCVR Trigger Register                                  |
| 0xa0  | `STET`            | Shadow TX Empty Trigger Register                              |
| 0xa4  | `HTX`             | Halt TX Regster                                               |
| 0xa8  | `DMASA`           | DMA Software Acknowledge Register                             |
| 0xac  | `TCR`             | Transfer Control Register                                     |
| 0xb0  | `DE_EN`           | DE Enable Register                                            |
| 0xb4  | `RE_EN`           | RE Enable Register                                            |
| 0xb8  | `DET`             | DE Assertion Time Register                                    |
| 0xbc  | `TAT`             | Turn-Around Time Register                                     |
| 0xc0  | `DLF`             | Divisor Latch (Fractional) Register                           |
| 0xc4  | `RAR`             | Receive-Mode Address Register                                 |
| 0xc8  | `TAR`             | Transmit-Mode Address Register                                |
| 0xcc  | `LCR_EXT`         | Line Control Register (Extended)                              |
| 0xd0  | `reserved2[9]`    |                                                               |
| 0xf4  | `CPR`             | Component Parameter Register                                  |
| 0xf8  | `UCV`             | UART Component Version                                        |
| 0xfc  | `CTR`             | Component Type Register                                       |

This block is replicated for all 3 UART peripherals. The latter registers (0x20
and higher) are not used in the SDK, but defined in the structure.

It looks like this matches the "Designware" 16550 compatible UART, which has a
driver in the Linux kernel tree (`drivers/tty/serial/8250/8250_dw.c`).

The best match to the register names and offsets appears to be the UART described in
[HUGEIC Communication Interface Peripheral User’s Guide](http://www.huge-ic.com/Communication%20Interface%20Peripheral%20User%27s%20Guide%20V0.1.pdf).

```
UART1: Designware UART version 4.0.1
UART1: Features APB 32 bits THRE SIR ADDITIONAL_FEAT SHADOW UART_ADD_ENCODED_PARAMS DMA_EXTRA UART 16 bytes
UART1: CTR 44570110
```

UARTHS
------

(source: `lib/drivers/include/uarths.h`)

| Ofs   | Name     | Description                     |
| ----- | -------- | ------------------------------- |
| 0x000 | `txdata` | Transmit data register          |
| 0x004 | `rxdata` | Receive data register           |
| 0x008 | `txctrl` | Transmit control register       |
| 0x00c | `rxctrl` | Receive control register        |
| 0x010 | `ie`     | UART interrupt enable           |
| 0x014 | `ip`     | UART Interrupt pending          |
| 0x018 | `div`    | Baud rate divisor               |

WDTx
----

(source: `lib/drivers/include/wdt.h`)

| Ofs   | Name              | Description                                                   |
| ----- | ----------------- | ------------------------------------------------------------- |
| 0x00  | `cr`              | WDT Control Register                                          |
| 0x04  | `torr`            | WDT Timeout Range Register                                    |
| 0x08  | `ccvr`            | WDT Current Counter Value Register                            |
| 0x0c  | `crr`             | WDT Counter Restart Register                                  |
| 0x10  | `stat`            | WDT Interrupt Status Register                                 |
| 0x14  | `eoi`             | WDT Interrupt Clear Register                                  |
| 0x18  | `resv1`           |                                                               |
| 0x1c  | `prot_level`      | WDT Protection level Register                                 |
| 0x20  | `resv4[49]`       |                                                               |
| 0xe4  | `comp_param_5`    | WDT Component Parameters Register 5                           |
| 0xe8  | `comp_param_4`    | WDT Component Parameters Register 4                           |
| 0xec  | `comp_param_3`    | WDT Component Parameters Register 3                           |
| 0xf0  | `comp_param_2`    | WDT Component Parameters Register 2                           |
| 0xf4  | `comp_param_1`    | WDT Component Parameters Register 1                           |
| 0xf8  | `comp_version`    | WDT Component Version Register                                |
| 0xfc  | `comp_type`       | WDT Component Type Register                                   |

This block is replicated for all 2 WDT peripherals.
