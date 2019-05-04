#include <stdio.h>
#include <string.h>
#include "sysctl.h"
#include "sleep.h"
#include "uarths.h"

typedef int otp_read_func(uint32_t offset, uint8_t *dest, uint32_t size);
static otp_read_func *otp_read_inner = (otp_read_func*)0x8800453c; /* fixed address in ROM */

static uint8_t ih_chksum;

void ih_start(void)
{
    printf(":");
    ih_chksum = 0;
}

void ih_emit(uint8_t val)
{
    printf("%02X", val);
    ih_chksum += val;
}

void ih_end()
{
    printf("%02X\n", (-ih_chksum) & 0xff);
}

int main(void)
{
    uint64_t core_id = current_coreid();

    if (core_id == 0)
    {
        int rv;

        sysctl_pll_set_freq(SYSCTL_PLL0, 800000000UL);
        sysctl_pll_set_freq(SYSCTL_PLL1, 300000000UL);
        sysctl_pll_set_freq(SYSCTL_PLL2, 45158400UL);

        uarths_init();

        /* system start, sleep a bit to allow UART clients to connect */
        usleep(100000);

        /* output in Intel HEX */
        uint8_t buf[32];
        for(uint32_t base=0; base<16384; base+=sizeof(buf)) {
            memset(buf, 0, sizeof(buf));
            rv = otp_read_inner(base, buf, sizeof(buf));
            if (rv != 0) {
                printf("warning: non-zero status %d while reading %08x..%08x\n", rv, base, (uint32_t)(base + sizeof(buf) - 1));
            }
            ih_start();
            ih_emit(sizeof(buf));
            ih_emit(base >> 8);
            ih_emit(base);
            ih_emit(0); /* Data */
            for (uint32_t x=0; x<sizeof(buf); ++x) {
                ih_emit(buf[x]);
            }
            ih_end();
        }
        ih_start();
        ih_emit(0);
        ih_emit(0);
        ih_emit(0);
        ih_emit(1); /* End Of File */
        ih_end();
    }

    while (1)
        asm volatile ("wfi");

    return 0;
}
