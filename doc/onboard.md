Peripherals
===========

List of on-board peripherals and chips for the various boards with a K210.

Sipeed M1(w) module
-------------------

Also called "Dan" or "LicheeDan".

(source: https://www.twblogs.net/a/5bde1d0f2b717720b51b61a1)

- K210 - CPU
- GD25LQ128D - 128Mbit w25qxx Flash chip (seems hardwired on SPI3)
- RY1303 - 3 Channel 5.5V 2A 1.5MHz DC/DC Step down PMU
- PT8211 - DAC audio (I2S)

M1w only:

- Espressif ESP8285 - WiFi module (serial, connected to FPIOA pin 6/7/8), this is a ESP8266 with built-in 1M flash

Maix Go
-------

(source: schematic)

### On-board

- MSA300 - Accelerometer (I2C, address 0x26\*)
- STM32F103C8 - JTAG & UART, debug M1 without extra Jlink, this bypasses the CH340C on the module (USB to host, serial to K210)
- OV2640 - Color CMOS UXGA (2.0 MegaPixel) CAMERA C HI (DVP, SCCB address 0x60)
- MSM261S4030H0 - Microphone (I2S)

### External

(source: schematic)

- ST7789V - 240x320 262K Color Single-Chip TFT Controller/Driver with Frame Memory (SPI)
- NS2009 - 4-Wire Touch Screen Controller (I2C, address 0x48\*)
- TF card slot (SPI)

\* Note regarding I2C addresses: the I2C peripheral automatically adds a LSB for read/write to the address,
  so 0x26 is and 0x4c and 0x4d, and 0x48 is 0x90 and 0x91.
