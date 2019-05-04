/* Copyright 2019 W.J. van der Laan, based on original DVP sample
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
#include <stdio.h>
#include <string.h>
#include "dvp.h"
#include "fpioa.h"
#include "lcd.h"
#include "ov5640.h"
#include "ov2640.h"
#include "plic.h"
#include "sysctl.h"
#include "uarths.h"
#include "nt35310.h"
#include "board_config.h"
#include "cp437_8x8.h"
#include "gpiohs.h"
#include "sleep.h"

#define PIN_KEY0 16
#define PIN_KEY1 15
#define PIN_KEY2 17
#define GPIO_KEY 4

#define DISP_WIDTH 320
#define DISP_HEIGHT 240

uint32_t g_lcd_gram0[38400] __attribute__((aligned(64)));
uint32_t g_lcd_gram1[38400] __attribute__((aligned(64)));

volatile uint8_t g_dvp_finish_flag;
volatile uint8_t g_ram_mux;
volatile int key_flag = 0;
volatile int key_state = 1;

uint16_t inbuf[DISP_WIDTH*DISP_HEIGHT];
uint16_t outbuf[DISP_WIDTH*DISP_HEIGHT];

static int on_irq_dvp(void *ctx)
{
    if (dvp_get_interrupt(DVP_STS_FRAME_FINISH))
    {
        /* switch gram */
        dvp_set_display_addr(g_ram_mux ? (uint32_t)g_lcd_gram0 : (uint32_t)g_lcd_gram1);

        // dvp_clear_interrupt(DVP_STS_FRAME_FINISH);
        dvp_clear_interrupt(DVP_STS_FRAME_START | DVP_STS_FRAME_FINISH);
        g_dvp_finish_flag = 1;
    }
    else
    {
        if (g_dvp_finish_flag == 0)
            dvp_start_convert();
        dvp_clear_interrupt(DVP_STS_FRAME_START);
    }

    return 0;
}

/**
 *	dvp_pclk	io47
 *	dvp_xclk	io46
 *	dvp_hsync	io45
 *	dvp_pwdn	io44
 *	dvp_vsync	io43
 *	dvp_rst		io42
 *	dvp_scl		io41
 *	dvp_sda		io40
 * */

/**
 *  lcd_cs	    36
 *  lcd_rst	37
 *  lcd_dc	    38
 *  lcd_wr 	39
 * */

static void io_mux_init(void)
{

#if BOARD_LICHEEDAN
    /* Init DVP IO map and function settings */
    fpioa_set_function(42, FUNC_CMOS_RST);
    fpioa_set_function(44, FUNC_CMOS_PWDN);
    fpioa_set_function(46, FUNC_CMOS_XCLK);
    fpioa_set_function(43, FUNC_CMOS_VSYNC);
    fpioa_set_function(45, FUNC_CMOS_HREF);
    fpioa_set_function(47, FUNC_CMOS_PCLK);
    fpioa_set_function(41, FUNC_SCCB_SCLK);
    fpioa_set_function(40, FUNC_SCCB_SDA);

    /* Init SPI IO map and function settings */
    fpioa_set_function(37, FUNC_GPIOHS0 + RST_GPIONUM);
    fpioa_set_function(38, FUNC_GPIOHS0 + DCX_GPIONUM);
    fpioa_set_function(36, FUNC_SPI0_SS3);
    fpioa_set_function(39, FUNC_SPI0_SCLK);

    sysctl_set_spi0_dvp_data(1);
#else
    /* Init DVP IO map and function settings */
    fpioa_set_function(11, FUNC_CMOS_RST);
    fpioa_set_function(13, FUNC_CMOS_PWDN);
    fpioa_set_function(14, FUNC_CMOS_XCLK);
    fpioa_set_function(12, FUNC_CMOS_VSYNC);
    fpioa_set_function(17, FUNC_CMOS_HREF);
    fpioa_set_function(15, FUNC_CMOS_PCLK);
    fpioa_set_function(10, FUNC_SCCB_SCLK);
    fpioa_set_function(9, FUNC_SCCB_SDA);

    /* Init SPI IO map and function settings */
    fpioa_set_function(8, FUNC_GPIOHS0 + DCX_GPIONUM);
    fpioa_set_function(6, FUNC_SPI0_SS3);
    fpioa_set_function(7, FUNC_SPI0_SCLK);

    sysctl_set_spi0_dvp_data(1);

#endif
}

static void io_set_power(void)
{
#if BOARD_LICHEEDAN
    /* Set dvp and spi pin to 1.8V */
    sysctl_set_power_mode(SYSCTL_POWER_BANK6, SYSCTL_POWER_V18);
    sysctl_set_power_mode(SYSCTL_POWER_BANK7, SYSCTL_POWER_V18);
#else
    /* Set dvp and spi pin to 1.8V */
    sysctl_set_power_mode(SYSCTL_POWER_BANK1, SYSCTL_POWER_V18);
    sysctl_set_power_mode(SYSCTL_POWER_BANK2, SYSCTL_POWER_V18);
#endif
}

/* helper functions */
static inline uint8_t r_from_rgb565(uint16_t iv)
{
    return ((iv >> 11) & 0x1f) << 3;
}
static inline uint8_t g_from_rgb565(uint16_t iv)
{
    return ((iv >> 5) & 0x3f) << 2;
}
static inline uint8_t b_from_rgb565(uint16_t iv)
{
    return ((iv >> 0) & 0x1f) << 3;
}
static inline uint16_t rgb565(uint8_t r, uint8_t g, uint8_t b)
{
    return ((r >> 3) << 11) | ((g >> 2) << 5) | (b >> 3);
}

/**
 * 2D convolution filter (box filter currently, would be more efficient with
 * 2-pass but we want to compute some positions only)
 */
uint16_t filter(int32_t bx, int32_t by, int32_t ksize)
{
    uint32_t r = 0;
    uint32_t g = 0;
    uint32_t b = 0;
    uint32_t n = 0;
    int32_t c = ksize/2;
    for (int32_t iy=0; iy<ksize; ++iy) {
        for (int32_t ix=0; ix<ksize; ++ix) {
            int32_t y = by + iy - c;
            int32_t x = bx + ix - c;
            if (x >= 0 && y >= 0 && x < DISP_WIDTH && y < DISP_HEIGHT) {
                uint16_t iv = inbuf[y*DISP_WIDTH + x];
                r += r_from_rgb565(iv);
                g += g_from_rgb565(iv);
                b += b_from_rgb565(iv);
                n += 1;
            }
        }
    }
    if (n) {
        r /= n;
        g /= n;
        b /= n;
    }
    return rgb565(r, g, b);
}

void irq_gpiohs2(void *gp)
{
    key_flag = 1;
    key_state = gpiohs_get_pin(GPIO_KEY);
    return;
}

static const uint32_t MODE_CNT = 3;
int main(void)
{
    uint64_t core_id = current_coreid();

    if (core_id == 0)
    {
        /* Set CPU and dvp clk */
        sysctl_pll_set_freq(SYSCTL_PLL0, 800000000UL);
        sysctl_pll_set_freq(SYSCTL_PLL1, 300000000UL);
        sysctl_pll_set_freq(SYSCTL_PLL2, 45158400UL);

        uarths_init();
        io_mux_init();
        io_set_power();
        plic_init();

        /* LCD init */
        printf("LCD init\n");
        lcd_init();
#if BOARD_LICHEEDAN
#if OV5640
        lcd_set_direction(DIR_YX_RLUD);
#else
        lcd_set_direction(DIR_YX_RLDU);
#endif
#else
#if OV5640
        lcd_set_direction(DIR_YX_LRUD);
#else
        lcd_set_direction(DIR_YX_LRDU);
#endif
#endif

        lcd_clear(BLACK);

        /* KEY init */
        fpioa_set_function(PIN_KEY0, FUNC_GPIOHS0 + GPIO_KEY);
        gpiohs_set_drive_mode(GPIO_KEY, GPIO_DM_INPUT);
        gpiohs_set_pin_edge(GPIO_KEY, GPIO_PE_BOTH);
        gpiohs_set_irq(GPIO_KEY, 1, irq_gpiohs2);

        /* DVP init */
        printf("DVP init\n");
#if OV5640
        dvp_init(16);
        dvp_enable_burst();
        dvp_set_output_enable(0, 1);
        dvp_set_output_enable(1, 1);
        dvp_set_image_format(DVP_CFG_RGB_FORMAT);
        dvp_set_image_size(320, 240);
        ov5640_init();
#else
        dvp_init(8);
        dvp_set_xclk_rate(24000000);
        dvp_enable_burst();
        dvp_set_output_enable(0, 1);
        dvp_set_output_enable(1, 1);
        dvp_set_image_format(DVP_CFG_RGB_FORMAT);
        dvp_set_image_size(320, 240);
        ov2640_init();
#endif

        dvp_set_ai_addr((uint32_t)0x40600000, (uint32_t)0x40612C00, (uint32_t)0x40625800);
        dvp_set_display_addr((uint32_t)g_lcd_gram0);
        dvp_config_interrupt(DVP_CFG_START_INT_ENABLE | DVP_CFG_FINISH_INT_ENABLE, 0);
        dvp_disable_auto();

        /* DVP interrupt config */
        printf("DVP interrupt config\r\n");
        plic_set_priority(IRQN_DVP_INTERRUPT, 1);
        plic_irq_register(IRQN_DVP_INTERRUPT, on_irq_dvp, NULL);
        plic_irq_enable(IRQN_DVP_INTERRUPT);

        /* enable global interrupt */
        sysctl_enable_irq();

        /* system start */
        printf("system start\r\n");
        g_ram_mux = 0;
        g_dvp_finish_flag = 0;
        dvp_clear_interrupt(DVP_STS_FRAME_START | DVP_STS_FRAME_FINISH);
        dvp_config_interrupt(DVP_CFG_START_INT_ENABLE | DVP_CFG_FINISH_INT_ENABLE, 1);

        uint32_t mode = 0;
        while (1)
        {
            /* ai cal finish*/
            while (g_dvp_finish_flag == 0)
                asm volatile ("wfi");
            g_dvp_finish_flag = 0;

            g_ram_mux ^= 0x01;

            /* word-swap input from camera */
            uint32_t *buf_32 = g_ram_mux ? g_lcd_gram0 : g_lcd_gram1;
            for (size_t x=0; x<DISP_WIDTH*DISP_HEIGHT/2; ++x) {
                uint32_t val = buf_32[x];
                inbuf[x*2+0] = val >> 16;
                inbuf[x*2+1] = val & 0xffff;
            }

            for (uint32_t cell_y=0; cell_y<30; ++cell_y) {
                for (uint32_t cell_x=0; cell_x<40; ++cell_x) {
                    uint32_t by = cell_y * 8;
                    uint32_t bx = cell_x * 8;

                    /* get average value */
                    uint32_t r = 0;
                    uint32_t g = 0;
                    uint32_t b = 0;
                    for (uint32_t iy=0; iy<8; ++iy) {
                        for (uint32_t ix=0; ix<8; ++ix) {
                            uint16_t iv = inbuf[(by + iy)*DISP_WIDTH + (bx + ix)];
                            r += r_from_rgb565(iv);
                            g += g_from_rgb565(iv);
                            b += b_from_rgb565(iv);
                        }
                    }
                    r /= 64;
                    g /= 64;
                    b /= 64;
                    uint16_t iv = rgb565(r, g, b);
                    uint8_t intensity = (r + g + b)/3;

                    uint8_t ch = glyph_by_fill[intensity];
                    const uint8_t *chdata = font[ch];

                    for (uint32_t iy=0; iy<8; ++iy) {
                        for (uint32_t ix=0; ix<8; ++ix) {
                            uint32_t y = by + iy;
                            uint32_t x = bx + ix;
                            uint16_t ov;
                            if (chdata[iy] & (1<<ix)) {
                                switch (mode) {
                                case 0: ov = iv; break;
                                case 1: ov = filter(x, y, 5); break;
                                case 2: ov = PURPLE; break;
                                default: ov = 0;
                                }
                            } else {
                                ov = 0;
                            }
                            outbuf[y*DISP_WIDTH + x] = ov;
                        }
                    }
                }
            }

            /* word-swap output to display */
            for (size_t x=0; x<DISP_WIDTH*DISP_HEIGHT/2; ++x) {
                buf_32[x] = (outbuf[x*2+0] << 16) | outbuf[x*2+1];
            }

            lcd_draw_picture(0, 0, DISP_WIDTH, DISP_HEIGHT, buf_32);

            if (key_flag)
            {
                if (key_state == 0)
                {
                    msleep(20);
                    key_flag = 0;
                    mode = (mode + 1) % MODE_CNT;
                    printf("mode: %d\n", mode);
                }
                else
                {
                    msleep(20);
                    key_flag = 0;
                }
            }
        }
    }
    while (1)
        asm volatile ("wfi");

    return 0;
}
