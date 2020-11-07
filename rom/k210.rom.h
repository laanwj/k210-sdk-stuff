/*

88000000-8801ffff

aes_get_data_in_flag            88002060    44
aes_init                        88001a80    1300
aes_process                     88002488    1048
aes_process_outer               880028a0    364
aes_read_out_data               880020e4    40
aes_write_aad                   88001f94    52
aes_write_text                  88001fc8    52
boot_main                       8800a5fc    1112
clint_ipi_send                  88002a84    84
entry                           88000000    140
entry_hart0                     88000124    24
exit                            88001a04    64
fcn.8800013c                    8800013c    332
fcn.88000288                    88000288    92
fcn.880002e4                    880002e4    332
fcn.88000430                    88000430    92
fcn.8800048c                    8800048c    328
fcn.880005d4                    880005d4    100
fcn.88000638                    88000638    196
fcn.880006fc                    880006fc    220
fcn.880007d8                    880007d8    820
fcn.8800208c_aes                8800208c    44
fcn.880042c0_otp                880042c0    636
fcn.88004bd4_otp                88004bd4    188
fcn.88004e9c_otp_read           88004e9c    124
fcn.880052a0                    880052a0    236
fcn.88009bc8_slip               88009bc8    228
fcn.8800a160_slip               8800a160    416
flash_read                      880099b4    128
flash_read_mode0                88009410    332
flash_read_mode_1_2             8800955c    532
flash_spi0_config               88009a34    352
flash_spi0_init                 8800ae24    272
flash_spi3_config               88009770    424
fpioa_init                      88002b54    552
fpioa_set_function              88003618    292
fpioa_set_function_raw          88003220    1016
FUN_88002138                    88002138    808
FUN_8800acdc                    8800acdc    148
FUN_8800c67c                    8800c67c    512
gcm_clear_chk_tag               88002460    40
gcm_get_tag_chk                 8800210c    44
gcm_get_tag_in_flag             880020b8    44
gcm_write_tag                   88001ffc    100
handle_baud_rate                8800a050    112
handle_memory_boot              8800a0c0    112
handle_memory_write             88009e28    572
handle_nop                      8800a130    16
isp_run                         8800a300    764
memcpy                          8800b32c    190
memset                          8800b3ea    108
otp_check_fuse_a_bit            88003b20    76
otp_check_fuse_b_bit            88004d58    200
otp_clear_0c                    88003afc    36
otp_clear_state18               88003b6c    180
otp_read                        8800476c    196
otp_read_inner                  8800453c    364
otp_reset                       880039b4    208
otp_set_0c                      88003ad4    40
otp_set_state18                 88003c20    160
panic_handler_internal          88001a44    60
panic_printf                    8800172c    728
printk                          880016b0    124
serial_init                     88008cc0    280
sha256_final                    880056c4    288
sha256_init                     8800538c    428
sha256_update                   88005538    396
slip_handle_pkt                 88009da4    144
slip_sendch                     8800aa54    128
slip_sendinner                  8800ab00    108
slip_sendpkt                    8800ab6c    72
slip_start                      8800aad4    44
slip_unescape                   8800abb4    296
spi_flash_read_manufacturer_id  88009918    156
spi_flash_set_read_mode         88009b94    52
spi_receive_data_1              88008dd8    312
spi_receive_data_2              8800902c    308
spi_send_data                   88008f10    284
strlen                          8800b494    18
sysctl_clock_bus_en             88005edc    276
sysctl_clock_device_en          88005ff0    2064
sysctl_clock_enable             88006800    104
sysctl_clock_get_clock_select   88007444    404
sysctl_clock_get_freq           88007d98    2884
sysctl_clock_get_threshold      88006e5c    836
sysctl_clock_set_clock_select   880071a0    676
sysctl_clock_set_threshold      880068d0    1420
sysctl_clock_source_get_freq    880075d8    212
sysctl_get_freq                 8800580c    36
sysctl_pll_clear_slip           88007784    208
sysctl_pll_enable               88007854    456
sysctl_pll_fast_enable_pll      88008a9c    204
sysctl_pll_get_freq             88007b18    640
sysctl_pll_is_lock              880076ac    216
sysctl_reset                    88005e90    76
sysctl_reset_ctl                88005830    1632
tfp_format                      88000b0c    2008
tfp_sprintf                     88001604    112
tfp_vsnprintf                   88001418    176
tfp_vsprintf                    88001594    112
turbo_mode_boot                 8800ad70    180
turbo_mode_boot                 8800af34    1016
uarths_getc                     88008bd0    80
uarths_putc                     88008b68    104
uarths_putchar                  88008c20    60
uarths_puts                     88008c5c    100
uarths_set_baudrate             88009d10    148
w25qxx_enable_quad_mode         8800939c    116
w25qxx_read_status_reg1         88009264    156
w25qxx_read_status_reg2         88009300    156
w25qxx_write_enable             88009160    152
w25qxx_write_status_reg         880091f8    108 

 */

typedef void V;typedef int I;typedef unsigned int uI; typedef long long J;typedef unsigned long long U; typedef char C;

typedef unsigned char   undef;

typedef unsigned int    uint;
typedef unsigned char    undef1;
typedef unsigned short    undef2;
typedef unsigned int    undef4;
typedef unsigned long long    undef8;

#define unkbyte9   unsigned long long
#define unkbyte10   unsigned long long
#define unkbyte11   unsigned long long
#define unkbyte12   unsigned long long
#define unkbyte13   unsigned long long
#define unkbyte14   unsigned long long
#define unkbyte15   unsigned long long
#define unkbyte16   unsigned long long

#define unkuint9   unsigned long long
#define unkuint10   unsigned long long
#define unkuint11   unsigned long long
#define unkuint12   unsigned long long
#define unkuint13   unsigned long long
#define unkuint14   unsigned long long
#define unkuint15   unsigned long long
#define unkuint16   unsigned long long

#define unkint9   long long
#define unkint10   long long
#define unkint11   long long
#define unkint12   long long
#define unkint13   long long
#define unkint14   long long
#define unkint15   long long
#define unkint16   long long

#define unkfloat1   float
#define unkfloat2   float
#define unkfloat3   float
#define unkfloat5   double
#define unkfloat6   double
#define unkfloat7   double
#define unkfloat9   long double
#define unkfloat11   long double
#define unkfloat12   long double
#define unkfloat13   long double
#define unkfloat14   long double
#define unkfloat15   long double
#define unkfloat16   long double

#define BADSPACEBASE   void
#define code   void

void entry_hart0(void);
void fcn.8800013c(ulonglong param_1,byte *param_2);
void fcn.88000288(longlong param_1,longlong param_2);
void fcn.880002e4(ulonglong param_1,byte *param_2);
void fcn.88000430(longlong param_1,longlong param_2);
void fcn.8800048c(uint param_1,byte *param_2);
void fcn.880005d4(int param_1,longlong param_2);
longlong fcn.88000638(byte param_1);
ulonglong fcn.880006fc(byte param_1,byte **param_2,int param_3,int *param_4);
void fcn.880007d8(undefined8 param_1,ulonglong param_2,byte *param_3);
void tfp_format(undefined8 param_1,ulonglong param_2,byte *param_3,char **param_4);
longlong tfp_vsnprintf(longlong param_1,longlong param_2,undefined8 param_3,undefined8 param_4);
longlong tfp_vsprintf(longlong param_1,undefined8 param_2,undefined8 param_3);
longlong tfp_sprintf(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,undefined8 param_5,undefined8 param_6,undefined8 param_7,undefined8 param_8);
undefined8 printk(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,undefined8 param_5,undefined8 param_6,undefined8 param_7,undefined8 param_8);
void panic_printf(undefined8 *param_1,undefined8 param_2,undefined8 param_3,longlong param_4);
void exit(int param_1);
undefined8 panic_handler_internal(undefined8 param_1,uint3 *param_2,ulonglong param_3,longlong param_4,int param_5,undefined4 param_6,uint param_7);
undefined8 aes_init(uint3 *param_1,byte param_2,uint3 *param_3,byte param_4,longlong param_5,int param_6,undefined4 param_7,uint param_8);
undefined8 aes_write_aad(undefined4 param_1);
undefined8 aes_write_text(undefined4 param_1);
undefined8 gcm_write_tag(undefined4 *param_1);
longlong aes_get_data_in_flag(void);
longlong fcn.8800208c_aes(void);
longlong gcm_get_tag_in_flag(void);
longlong aes_read_out_data(void);
longlong gcm_get_tag_chk(void);
undefined8 FUN_88002138(undefined *param_1);
undefined8 gcm_clear_chk_tag(void);
undefined8 aes_process(longlong param_1,longlong param_2,uint param_3,int param_4);
undefined8 aes_process_outer(longlong param_1,longlong param_2,uint param_3,int param_4);
undefined8 clint_ipi_send(ulonglong param_1);
undefined8 fpioa_init(void);
undefined8 fpioa_set_function_raw(int param_1,uint param_2);
undefined8 fpioa_set_function(uint param_1,uint param_2);
void otp_reset(byte param_1);
void otp_set_0c(void);
void otp_clear_0c(void);
undefined8 otp_check_fuse_a_bit(uint param_1);
undefined8 otp_clear_state18(void);
undefined8 otp_set_state18(void);
longlong fcn.880042c0_otp(undefined8 param_1,byte *param_2,undefined8 param_3);
undefined8 otp_read_inner(int param_1,undefined *param_2,int param_3);
longlong otp_read(uint param_1,undefined8 param_2,uint param_3);
longlong fcn.88004bd4_otp(uint param_1);
undefined8 otp_check_fuse_b_bit(uint param_1);
longlong fcn.88004e9c_otp_read(uint param_1);
ulonglong fcn.880052a0(undefined8 param_1);
undefined8 sha256_init(char param_1,char param_2,int param_3,undefined8 *param_4);
void sha256_update(longlong *param_1,longlong param_2,uint param_3);
void sha256_final(undefined8 *param_1,undefined4 *param_2);
undefined8 sysctl_get_freq(void);
void sysctl_reset_ctl(undefined4 param_1,byte param_2);
void sysctl_reset(int param_1);
undefined8 sysctl_clock_bus_en(undefined4 param_1,byte param_2);
undefined8 sysctl_clock_device_en(undefined4 param_1,byte param_2);
undefined8 sysctl_clock_enable(uint param_1);
undefined8 sysctl_clock_set_threshold(undefined4 param_1,uint param_2);
longlong sysctl_clock_get_threshold(undefined4 param_1);
undefined8 sysctl_clock_set_clock_select(undefined4 param_1,uint param_2);
longlong sysctl_clock_get_clock_select(undefined4 param_1);
longlong sysctl_clock_source_get_freq(undefined4 param_1);
undefined8 sysctl_pll_is_lock(uint param_1);
undefined8 sysctl_pll_clear_slip(uint param_1);
undefined8 sysctl_pll_enable(uint param_1);
longlong sysctl_pll_get_freq(uint param_1);
longlong sysctl_clock_get_freq(undefined4 param_1);
undefined8 sysctl_pll_fast_enable_pll(void);
undefined8 uarths_putc(byte param_1);
longlong uarths_getc(void);
undefined8 uarths_putchar(byte param_1);
undefined8 uarths_puts(byte *param_1);
undefined8 serial_init(void);
void spi_receive_data_1(byte *param_1,char param_2,undefined *param_3,uint param_4);
void spi_send_data(byte *param_1,char param_2,byte *param_3,int param_4);
void spi_receive_data_2(undefined4 *param_1,char param_2,undefined *param_3,uint param_4);
void w25qxx_write_enable(void);
void w25qxx_write_status_reg(undefined param_1,undefined param_2);
void w25qxx_read_status_reg1(undefined8 param_1);
void w25qxx_read_status_reg2(undefined8 param_1);
void w25qxx_enable_quad_mode(void);
void flash_read_mode0(int param_1,longlong param_2,uint param_3);
void flash_read_mode_1_2(int param_1,longlong param_2,uint param_3);
void flash_spi3_config(void);
void spi_flash_read_manufacturer_id(undefined8 param_1);
void flash_read(int param_1,undefined8 param_2,int param_3);
void flash_spi0_config(void);
void spi_flash_set_read_mode(undefined param_1);
longlong fcn.88009bc8_slip(byte *param_1,int param_2);
void uarths_set_baudrate(uint param_1);
void slip_handle_pkt(int *param_1,ushort *param_2,longlong param_3);
void handle_memory_write(void);
void handle_baud_rate(void);
void handle_memory_boot(void);
void handle_nop(void);
undefined8 fcn.8800a160_slip(longlong param_1,byte param_2);
void isp_run(void);
void boot_main(void);
void slip_sendch(byte param_1);
void slip_start(void);
void slip_sendinner(longlong param_1,ulonglong param_2);
void slip_sendpkt(undefined8 param_1,ulonglong param_2);
longlong slip_unescape(byte param_1,int *param_2);
void FUN_8800acdc(uint param_1);
undefined8 turbo_mode_boot(void);
undefined8 flash_spi0_init(void);
void turbo_mode_boot(void);
void memcpy(undefined8 *param_1,undefined8 *param_2,ulonglong param_3);
void memset(ulonglong *param_1,ulonglong param_2,ulonglong param_3);
char * strlen(char *param_1);
void FUN_8800c67c(void);

//:~
