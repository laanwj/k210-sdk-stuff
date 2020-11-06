//! ST7789V LCD driver
use crate::soc::gpio;
use crate::soc::gpiohs;
use crate::soc::sleep::usleep;
use crate::soc::spi::{SPI,work_mode,frame_format,aitm,tmod};
use crate::soc::dmac::{DMAC,dma_channel};

// These are the values used in the Kendryte SDK but should not ideally be hardcoded here, but
// defined in the main.rs and passed to the constructor
// (they're arbitrarily configurable by setting
//    fpioa::set_function(io::LCD_CS, fpioa::function::SPI0_SS[0-3]);
//    fpioa::set_function(io::LCD_RST, fpioa::function::gpiohs(lcd::RST_GPIONUM));
//    fpioa::set_function(io::LCD_DC, fpioa::function::gpiohs(lcd::DCX_GPIONUM));
// )
pub const SPI_CS: u32 = 3;
pub const DCX_GPIONUM: u8 = 2;
pub const RST_GPIONUM: u8 = 3;
pub const LCD_X_MAX: u16 = 240;
pub const LCD_Y_MAX: u16 = 320;
/** SPI clock (this seems to be the highest possible value which is reliable on both my MaixGo
 * boards) */
pub const SPI_CLK: u32 = 18_000_000;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum command {
    NOP = 0x00,
    SWRESET = 0x01,
    RDDID = 0x04,
    RDDST = 0x09,
    RDDPM = 0x0A,
    RDDMADCTL = 0x0B,
    RDDCOLMOD = 0x0C,
    RDDIM = 0x0D,
    RDDSM = 0x0E,
    RDDSDR = 0x0F,
    SLPIN = 0x10,
    SLPOUT = 0x11,
    PTLON = 0x12,
    NORON = 0x13,
    INVOF = 0x20,
    INVON = 0x21,
    GAMSET = 0x26,
    DISPOFF = 0x28,
    DISPON = 0x29,
    CASET = 0x2A,
    RASET = 0x2B,
    RAMWR = 0x2C,
    RAMRD = 0x2E,
    PTLAR = 0x30,
    VSCRDEF = 0x33,
    TEOFF = 0x34,
    TEON = 0x35,
    MADCTL = 0x36,
    VSCRSADD = 0x37,
    IDMOFF = 0x38,
    IDMON = 0x39,
    COLMOD = 0x3A,
    RAMWRC = 0x3C,
    RAMRDC = 0x3E,
    TESCAN = 0x44,
    RDTESCAN = 0x45,
    WRDISBV = 0x51,
    RDDISBV = 0x52,
    WRCTRLD = 0x53,
    RDCTRLD = 0x54,
    WRCACE = 0x55,
    RDCABC = 0x56,
    WRCABCMB = 0x5E,
    RDCABCMB = 0x5F,
    RDABCSDR = 0x68,
    RAMCTRL = 0xB0,
    RGBCTRL = 0xB1,
    PORCTRL = 0xB2,
    FRCTRL1 = 0xB3,
    PARCTRL = 0xB5,
    GCTRL = 0xB7,
    GTADJ = 0xB8,
    DGMEN = 0xBA,
    VCOMS = 0xBB,
    LCMCTRL = 0xC0,
    IDSET = 0xC1,
    VDVVRHEN = 0xC2,
    VRHS = 0xC3,
    VDVSET = 0xC4,
    VCMOFSET = 0xC5,
    FRCTR2 = 0xC6,
    CABCCTRL = 0xC7,
    REGSEL1 = 0xC8,
    REGSEL2 = 0xCA,
    PWMFRSEL = 0xCC,
    PWCTRL1 = 0xD0,
    VAPVANEN = 0xD2,
    RDID1 = 0xDA,
    RDID2 = 0xDB,
    RDID3 = 0xDC,
    CMD2EN = 0xDF,
    PVGAMCTRL = 0xE0,
    NVGAMCTRL = 0xE1,
    DGMLUTR = 0xE2,
    DGMLUTB = 0xE3,
    GATECTRL = 0xE4,
    SPI2EN = 0xE7,
    PWCTRL2 = 0xE8,
    EQCTRL = 0xE9,
    PROMCTRL = 0xEC,
    PROMEN = 0xFA,
    NVMSET = 0xFC,
    PROMACT = 0xFE,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum direction {
    XY_RLUD = 0x00,
    YX_RLUD = 0x20,
    XY_LRUD = 0x40,
    YX_LRUD = 0x60,
    XY_RLDU = 0x80,
    YX_RLDU = 0xA0,
    XY_LRDU = 0xC0,
    YX_LRDU = 0xE0,
}
pub const DIR_XY_MASK: u8 = 0x20;
pub const DIR_MASK: u8 = 0xE0;

pub struct LCD<'a, SPI> {
    spi: SPI,
    spi_cs: u32,
    dcx_gpionum: u8,
    rst_gpionum: u8,
    dmac: &'a DMAC,
    channel: dma_channel,
    pub width: u16,
    pub height: u16,
}

/** Low-level interface */
pub trait LCDLL {
    fn hard_init(&self);
    fn write_command(&self, cmd: command);
    /** Write bytes. These are provided as 32-bit units (ignoring the upper 24 bits) for efficient DMA */
    fn write_byte(&self, data_buf: &[u32]);
    /** Write 32-bit words. */
    fn write_word(&self, data_buf: &[u32]);
    fn fill_data(&self, data: u32, length: usize);
}

/** High-level interface */
pub trait LCDHL {
    /** Turn on and initialize the LCD display, this needs to be called before it's possible to use it. */
    fn init(&mut self);
    /** Set direction/alignment of display. It can be rotated and/or mirrored in every direction. */
    fn set_direction(&mut self, dir: direction);
    /** Clear the screen to a single RGB565 color. */
    fn clear(&self, color: u16);
    /** Draw a picture, filling the entire screen or part of it. `data` packs two RGB565 pixels
     * per u32 as 0xBBBBAAAA. */
    fn draw_picture(&self, x1: u16, y1: u16, width: u16, height: u16, data: &[u32]);
    /** Shut down and turn off the screen. */
    fn shutdown(&mut self);
}

impl<'a, X: SPI> LCD<'a, X> {
    pub fn new(spi: X, dmac: &'a DMAC, channel: dma_channel) -> Self {
        Self {
            spi,
            spi_cs: SPI_CS,
            dcx_gpionum: DCX_GPIONUM,
            rst_gpionum: RST_GPIONUM,
            dmac,
            channel,
            width: 0,
            height: 0,
        }
    }

    fn init_dcx(&self) {
        gpiohs::set_direction(self.dcx_gpionum, gpio::direction::OUTPUT);
        gpiohs::set_pin(self.dcx_gpionum, true);
    }

    fn set_dcx_control(&self) {
        gpiohs::set_pin(self.dcx_gpionum, false);
    }

    fn set_dcx_data(&self) {
        gpiohs::set_pin(self.dcx_gpionum, true);
    }

    fn init_rst(&self) {
        gpiohs::set_direction(self.rst_gpionum, gpio::direction::OUTPUT);
        gpiohs::set_pin(self.rst_gpionum, true);
    }

    fn set_rst(&self, val: bool) {
        gpiohs::set_pin(self.rst_gpionum, val);
    }

    fn set_area(&self, x1: u16, y1: u16, x2: u16, y2: u16) {
        self.write_command(command::CASET);
        self.write_byte(&[
            (x1 >> 8).into(),
            (x1 & 0xff).into(),
            (x2 >> 8).into(),
            (x2 & 0xff).into(),
        ]);

        self.write_command(command::RASET);
        self.write_byte(&[
            (y1 >> 8).into(),
            (y1 & 0xff).into(),
            (y2 >> 8).into(),
            (y2 & 0xff).into(),
        ]);

        self.write_command(command::RAMWR);
    }
}

/** Low-level functions */
impl<X: SPI> LCDLL for LCD<'_, X> {
    fn hard_init(&self) {
        self.init_dcx();
        self.init_rst();
        self.set_rst(false);
        self.spi.set_clk_rate(SPI_CLK);
        self.spi.configure(
            work_mode::MODE0,
            frame_format::OCTAL,
            8, /*data bits*/
            0, /*endian*/
            8, /*instruction length*/
            0, /*address length*/
            0, /*wait cycles*/
            aitm::AS_FRAME_FORMAT,
            tmod::TRANS,
        );
        self.set_rst(true);
    }

    fn write_command(&self, cmd: command) {
        self.set_dcx_control();
        self.spi.configure(
            work_mode::MODE0,
            frame_format::OCTAL,
            8, /*data bits*/
            0, /*endian*/
            8, /*instruction length*/
            0, /*address length*/
            0, /*wait cycles*/
            aitm::AS_FRAME_FORMAT,
            tmod::TRANS,
        );
        self.spi.send_data_dma(self.dmac, self.channel, self.spi_cs, &[cmd as u32]);
    }

    fn write_byte(&self, data_buf: &[u32]) {
        self.set_dcx_data();
        self.spi.configure(
            work_mode::MODE0,
            frame_format::OCTAL,
            8, /*data bits*/
            0, /*endian*/
            0, /*instruction length*/
            8, /*address length*/
            0, /*wait cycles*/
            aitm::AS_FRAME_FORMAT,
            tmod::TRANS,
        );
        self.spi.send_data_dma(self.dmac, self.channel, self.spi_cs, data_buf);
    }

    fn write_word(&self, data_buf: &[u32]) {
        self.set_dcx_data();
        self.spi.configure(
            work_mode::MODE0,
            frame_format::OCTAL,
            32, /*data bits*/
            1,  /*endian*/
            0,  /*instruction length*/
            32, /*address length*/
            0,  /*wait cycles*/
            aitm::AS_FRAME_FORMAT,
            tmod::TRANS,
        );
        self.spi.send_data_dma(self.dmac, self.channel, self.spi_cs, data_buf);
    }

    fn fill_data(&self, data: u32, length: usize) {
        self.set_dcx_data();
        self.spi.configure(
            work_mode::MODE0,
            frame_format::OCTAL,
            32, /*data bits*/
            0,  /*endian*/
            0,  /*instruction length*/
            32, /*address length*/
            0,  /*wait cycles*/
            aitm::AS_FRAME_FORMAT,
            tmod::TRANS,
        );
        self.spi.fill_data_dma(self.dmac, self.channel, self.spi_cs, data, length);
    }
}

/* High-level functions */
impl<X: SPI> LCDHL for LCD<'_, X> {
    fn init(&mut self) {
        self.hard_init();
        /*soft reset*/
        self.write_command(command::SWRESET);
        usleep(100000);
        /*exit sleep*/
        self.write_command(command::SLPOUT);
        usleep(100000);
        /*pixel format*/
        self.write_command(command::RAMCTRL);
        self.write_byte(&[0x00, 0xf0 | 0x08]); // little-endian

        self.write_command(command::COLMOD);
        self.write_byte(&[0x55]);
        self.set_direction(direction::XY_LRUD);

        /*display on*/
        self.write_command(command::DISPON);
    }

    fn shutdown(&mut self) {
        self.set_rst(false);
    }

    fn set_direction(&mut self, dir: direction) {
        if ((dir as u8) & DIR_XY_MASK) != 0 {
            self.width = LCD_Y_MAX;
            self.height = LCD_X_MAX;
        } else {
            self.width = LCD_X_MAX;
            self.height = LCD_Y_MAX;
        }

        self.write_command(command::MADCTL);
        self.write_byte(&[dir as u32]);
    }

    fn clear(&self, color: u16) {
        let data = (u32::from(color) << 16) | u32::from(color);

        self.set_area(0, 0, self.width - 1, self.height - 1);
        self.fill_data(data, usize::from(LCD_X_MAX) * usize::from(LCD_Y_MAX) / 2);
    }

    fn draw_picture(&self, x1: u16, y1: u16, width: u16, height: u16, data: &[u32]) {
        self.set_area(x1, y1, x1 + width - 1, y1 + height - 1);
        assert!(data.len() == (width as usize) * (height as usize) / 2);
        self.write_word(data);
    }
}
