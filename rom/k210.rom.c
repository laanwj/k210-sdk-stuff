#include "k210.rom.h"

/* global entry:
88000000      csrrwi     zero,mideleg,0x0              # reset irq delegation
88000004      csrrwi     zero,medeleg,0x0              # reset exception delegation
88000008      csrrwi     zero,mie,0x0                  # disable machine irq
8800000c      csrrwi     zero,mip,0x0                  # reset pending machine irq
88000010      auipc      t0,0x0
88000014      addi       t0,t0,0x7c
88000018      csrrw      zero,mtvec,t0                 # machine trap handler
8800001c      lui        t0,0x6
88000020      csrrs      zero,mstatus,t0               # reset machine status
88000024      auipc      gp,-0x7a04
88000028      addi       gp,gp,0x7dc
8800002c      auipc      tp,-0x7a04
88000030      addi tp,tp,0x13
88000034      andi       tp,tp,-0x40
88000038      csrrs      a0,mhartid,zero               # switch to hart 0
8800003c      xori       a0,a0,0x1
88000040      addi       a0,a0,0x1
88000044      slli       a2,a0,0xd
88000048      add        tp,tp,a2
8800004c      mv         sp,tp
88000050      csrrs      t0,mhartid,zero
88000054      beq        t0,zero,LAB_88000088          # jump to entry_hart0
88000058      lui        t1,0x2000
8800005c      addiw      t1,t1,0x4
88000060      mv         t2,zero
88000064      sw         t2,0x0(t1=>DAT_02000004)
88000068      csrrsi     a5,mie,0x8

 LAB_8800006c                                          XREF[1]:     88000078(j)
8800006c      wfi
88000070      csrrs      a5,mip,zero
88000074      andi       a5,a5,0x8
88000078      beq        a5,zero,LAB_8800006c
8800007c      addiw      t1,zero,0x1
88000080      slli       t1,t1,0x1f
88000084      jalr       ra,t1=>SUB_80000000,0x0

 LAB_88000088                                          XREF[2]:     88000054(j),
                                                       entry_hart0:88000128(*)
88000088 6f 00 c0 09     j          entry_hart0

8800008c      addi       sp,sp,-0x100                  # reset regs
88000090      sd         ra,0x8(sp)
88000094      sd         sp,0x10(sp)
88000098      sd         gp,0x18(sp)
8800009c      sd         tp,0x20(sp)
880000a0      sd         t0,0x28(sp)
880000a4      sd         t1,0x30(sp)
880000a8      sd         t2,0x38(sp)
880000ac      sd         s0,0x40(sp)
880000b0      sd         s1,0x48(sp)
880000b4      sd         a0,0x50(sp)
880000b8      sd         a1,0x58(sp)
880000bc      sd         a2,0x60(sp)
880000c0      sd         a3,0x68(sp)
880000c4      sd         a4,0x70(sp)
880000c8      sd         a5,0x78(sp)
880000cc      sd         a6,0x80(sp)
880000d0      sd         a7,0x88(sp)
880000d4      sd         s2,0x90(sp)
880000d8      sd         s3,0x98(sp)
880000dc      sd         s4,0xa0(sp)
880000e0      sd         s5,0xa8(sp)
880000e4      sd         s6,0xb0(sp)
880000e8      sd         s7,0xb8(sp)
880000ec      sd         s8,0xc0(sp)
880000f0      sd         s9,0xc8(sp)
880000f4      sd         s10,0xd0(sp)
880000f8      sd         s11,0xd8(sp)
880000fc      sd         t3,0xe0(sp)
88000100      sd         t4,0xe8(sp)
88000104      sd         t5,0xf0(sp)
88000108      sd         t6,0xf8(sp)
8800010c      csrrs      a0,mcause,zero           # reset machine trap cause
88000110      csrrs      a1,mepc,zero
88000114      mv         a2,sp
88000118      addi       a3,sp,0x100
8800011c      jal        ra,panic_handler_internal

 halt:                               XREF[1]:     88000120(j)
88000120      j          halt

*/

void entry_hart0(void) {
  C*extraout_a1;
  C cVar1;
  U uVar2;
  U uStack72;
  C*pcStack48;
  U uStack40;

  serial_init();
  uStack72 = boot_main();
  uStack40 = 1;
  pcStack48 = *(C**)(extraout_a1 + 0x18);
  if ((*(I*)(extraout_a1 + 8) != 0) || (uStack72 != 0)) {
    while (((J) * (I*)(extraout_a1 + 0x10) & 0xffffffffU) <=
           uStack72 / uStack40) {
      uStack40 = uStack40 * ((J) * (I*)(extraout_a1 + 0x10) & 0xffffffffU);
    }
    while (uStack40 != 0) {
      uVar2 = uStack72 / uStack40;
      uStack72 = uStack72 % uStack40;
      uStack40 = uStack40 / ((J) * (I*)(extraout_a1 + 0x10) & 0xffffffffU);
      if ((I)uVar2 < 10) {
        cVar1 = '0';
      } else {
        if ((*extraout_a1 & 4) == 0) {
          cVar1 = 'W';
        } else {
          cVar1 = '7';
        }
      }
      *pcStack48 = cVar1 + (C)uVar2;
      pcStack48 = pcStack48 + 1;
    }
    *(C**)(extraout_a1 + 0x20) = pcStack48 + -*(J*)(extraout_a1 + 0x18);
  }
  R;
}

void fcn_8800013c(U a1, C*a2)
{
  C cVar1;
  U uVar2;
  U l38;
  C*l20;
  U l18;

  l18 = 1;
  l20 = *(C**)(a2 + 0x18);
  if ((*(I*)(a2 + 8) != 0) || (a1 != 0)) {
    while (l38 = a1, ((J) * (I*)(a2 + 0x10) & 0xffffffffU) <= a1 / l18) {
      l18 = l18 * ((J) * (I*)(a2 + 0x10) & 0xffffffffU);
    }
    while (l18 != 0) {
      uVar2 = l38 / l18;
      l38 = l38 % l18;
      l18 = l18 / ((J) * (I*)(a2 + 0x10) & 0xffffffffU);
      if ((I)uVar2 < 10) {
        cVar1 = '0';
      } else {
        if ((*a2 & 4) == 0) {
          cVar1 = 'W';
        } else {
          cVar1 = '7';
        }
      }
      *l20 = cVar1 + (C)uVar2;
      l20 = l20 + 1;
    }
    *(C**)(a2 + 0x20) = l20 + -*(J*)(a2 + 0x18);
  }
  R;
}

void fcn_88000288(J a1, J a2)
{
  J l18;

  l18 = a1;
  if (a1 < 0) {
    l18 = -a1;
    *(undef *)(a2 + 0xc) = 0x2d;
  }
  fcn_8800013c(l18, a2);
  R;
}

void fcn_880002e4(U a1, C*a2)
{
  C cVar1;
  U uVar2;
  U l38;
  C*l20;
  U l18;

  l18 = 1;
  l20 = *(C**)(a2 + 0x18);
  if ((*(I*)(a2 + 8) != 0) || (a1 != 0)) {
    while (l38 = a1, ((J) * (I*)(a2 + 0x10) & 0xffffffffU) <= a1 / l18) {
      l18 = l18 * ((J) * (I*)(a2 + 0x10) & 0xffffffffU);
    }
    while (l18 != 0) {
      uVar2 = l38 / l18;
      l38 = l38 % l18;
      l18 = l18 / ((J) * (I*)(a2 + 0x10) & 0xffffffffU);
      if ((I)uVar2 < 10) {
        cVar1 = '0';
      } else {
        if ((*a2 & 4) == 0) {
          cVar1 = 'W';
        } else {
          cVar1 = '7';
        }
      }
      *l20 = cVar1 + (C)uVar2;
      l20 = l20 + 1;
    }
    *(C**)(a2 + 0x20) = l20 + -*(J*)(a2 + 0x18);
  }
  R;
}

void fcn_88000430(J a1, J a2)
{
  J l18;

  l18 = a1;
  if (a1 < 0) {
    l18 = -a1;
    *(undef *)(a2 + 0xc) = 0x2d;
  }
  fcn_880002e4(l18, a2);
  R;
}

void fcn_8800048c(uI a1, C*a2)
{
  uI uVar1;
  C cVar2;
  uI l34;
  C*l20;
  uI l14;

  l14 = 1;
  l20 = *(C**)(a2 + 0x18);
  if ((*(I*)(a2 + 8) != 0) || (a1 != 0)) {
    while (l34 = a1, *(uI*)(a2 + 0x10) <= a1 / l14) {
      l14 = l14 * *(I*)(a2 + 0x10);
    }
    while (l14 != 0) {
      uVar1 = l34 / l14;
      l34 = l34 % l14;
      l14 = l14 / *(uI*)(a2 + 0x10);
      if ((I)uVar1 < 10) {
        cVar2 = '0';
      } else {
        if ((*a2 & 4) == 0) {
          cVar2 = 'W';
        } else {
          cVar2 = '7';
        }
      }
      *l20 = cVar2 + (C)uVar1;
      l20 = l20 + 1;
    }
    *(C**)(a2 + 0x20) = l20 + -*(J*)(a2 + 0x18);
  }
  R;
}

void fcn_880005d4(I a1, J a2)
{
  I l14;

  l14 = a1;
  if (a1 < 0) {
    l14 = -a1;
    *(undef *)(a2 + 0xc) = 0x2d;
  }
  fcn_8800048c((J)l14, a2);
  R;
}


J fcn_88000638(C a1)
{
  J lVar1;

  if ((a1 < 0x30) || (0x39 < a1)) {
    if ((a1 < 0x61) || (0x66 < a1)) {
      if ((a1 < 0x41) || (0x46 < a1)) {
        lVar1 = -1;
      } else {
        lVar1 = (J)(I)((uI)a1 - 0x37);
      }
    } else {
      lVar1 = (J)(I)((uI)a1 - 0x57);
    }
  } else {
    lVar1 = (J)(I)((uI)a1 - 0x30);
  }
  R lVar1;
}


U fcn_880006fc(C a1, C**a2, I a3, I*a4)
{
  I iVar1;
  C l21;
  I l1c;
  C*l18;

  l1c = 0;
  l21 = a1;
  l18 = *a2;
  while ((iVar1 = fcn_88000638((U)l21), -1 < iVar1 && (iVar1 <= a3))) {
    l1c = l1c * a3 + iVar1;
    l21 = *l18;
    l18 = l18 + 1;
  }
  *a2 = l18;
  *a4 = l1c;
  R(U) l21;
}


void fcn_880007d8(undef8 a1, U a2, C*a3)
{
  bool bVar1;
  I iVar2;
  undef8 uVar3;
  J l28;
  C*l20;
  I l18;
  I l14;

  l20 = *(C**)(a3 + 0x18);
  l28 = *(J*)(a3 + 0x20);
  l14 = *(I*)(a3 + 4) - (I)l28;
  l18 = *(I*)(a3 + 8) - (I)l28;
  if (a3[0xc] != 0) {
    l14 = l14 + -1;
  }
  if (((*a3 & 2) == 0) || (*(I*)(a3 + 0x10) != 0x10)) {
    if (((*a3 & 2) != 0) && (*(I*)(a3 + 0x10) == 8)) {
      l14 = l14 + -1;
    }
  } else {
    l14 = l14 + -2;
  }
  if (0 < l18) {
    l14 = l14 - l18;
  }
  if (((*a3 & 1) == 0) && ((*a3 & 8) == 0)) {
    while (iVar2 = l14 + -1, bVar1 = 0 < l14, l14 = iVar2, bVar1) {
      (*(code *)(a2 & 0xfffffffffffffffe))(a1, 0x20);
    }
  }
  if (a3[0xc] != 0) {
    (*(code *)(a2 & 0xfffffffffffffffe))(a1, (U)a3[0xc]);
  }
  if (((*a3 & 2) == 0) || (*(I*)(a3 + 0x10) != 0x10)) {
    if (((*a3 & 2) != 0) && (*(I*)(a3 + 0x10) == 8)) {
      (*(code *)(a2 & 0xfffffffffffffffe))(a1, 0x30);
    }
  } else {
    (*(code *)(a2 & 0xfffffffffffffffe))(a1, 0x30);
    if ((*a3 & 4) == 0) {
      uVar3 = 0x78;
    } else {
      uVar3 = 0x58;
    }
    (*(code *)(a2 & 0xfffffffffffffffe))(a1, uVar3);
  }
  while (0 < l18) {
    (*(code *)(a2 & 0xfffffffffffffffe))(a1, 0x30);
    l18 = l18 + -1;
  }
  if ((*a3 & 1) != 0) {
    while (iVar2 = l14 + -1, bVar1 = 0 < l14, l14 = iVar2, bVar1) {
      (*(code *)(a2 & 0xfffffffffffffffe))(a1, 0x30);
    }
  }
  while (l28 != 0) {
    if (*l20 == 0) break;
    (*(code *)(a2 & 0xfffffffffffffffe))(a1, (U)*l20);
    l28 = l28 + -1;
    l20 = l20 + 1;
  }
  if (((*a3 & 1) == 0) && ((*a3 & 8) != 0)) {
    while (0 < l14) {
      (*(code *)(a2 & 0xfffffffffffffffe))(a1, 0x20);
      l14 = l14 + -1;
    }
  }
  R;
}


void tfp_format(undef8 a1, U a2, C*a3, C**a4)
{
  C cVar1;
  C*pbVar2;
  C**ppcVar3;
  C*pcVar4;
  C**l90;
  C*l88;
  U l80;
  undef8 l78;
  I l70;
  I l6c;
  C acStack104[24];
  C l50[4];
  I l4c;
  I l48;
  undef l44;
  undef4 l40;
  C*l38;
  J l30;
  I l24;
  C*l20;
  I l18;
  C l12;
  C l11;

  l90 = a4;
  l88 = a3;
  l80 = a2;
  l78 = a1;
LAB_880012a8:
  while (true) {
    l11 = *l88;
    if (l11 == 0) {
      R;
    }
    if (l11 == 0x25) break;
    l88 = l88 + 1;
    (*(code *)(l80 & 0xfffffffffffffffe))(l78, (U)l11);
  }
  l12 = '\0';
  l50[0] = l50[0] & 0xf0;
  l4c = 0;
  l48 = -1;
  l44 = 0;
  l38 = acStack104;
  l30 = 0;
  l88 = l88 + 1;
  while (true) {
    pbVar2 = l88 + 1;
    l11 = *l88;
    if (l11 == 0) break;
    if (l11 == 0x2d) {
      l50[0] = l50[0] | 8;
      l88 = pbVar2;
    } else {
      if (l11 == 0x30) {
        l50[0] = l50[0] | 1;
        l88 = pbVar2;
      } else {
        if (l11 != 0x23) break;
        l50[0] = l50[0] | 2;
        l88 = pbVar2;
      }
    }
  }
  if ((l50[0] & 8) != 0) {
    l50[0] = l50[0] & 0xfe;
  }
  if (l11 == 0x2a) {
    l88 = l88 + 2;
    l11 = *pbVar2;
    ppcVar3 = l90 + 1;
    l4c = *(I*)l90;
    l90 = ppcVar3;
    if (l4c < 0) {
      l50[0] = l50[0] | 8;
      l4c = -l4c;
    }
  } else {
    l88 = pbVar2;
    if ((0x2f < l11) && (l11 < 0x3a)) {
      l11 = fcn_880006fc((U)l11, &l88, 10, &l6c);
      l4c = l6c;
    }
  }
  if (l11 == 0x2e) {
    pbVar2 = l88 + 1;
    l11 = *l88;
    if (l11 == 0x2a) {
      l88 = l88 + 2;
      l11 = *pbVar2;
      ppcVar3 = l90 + 1;
      l24 = *(I*)l90;
      l90 = ppcVar3;
      l48 = l24;
      if (l24 < 0) {
        l48 = -1;
      }
    } else {
      l88 = pbVar2;
      if ((l11 < 0x30) || (0x39 < l11)) {
        l48 = 0;
      } else {
        l11 = fcn_880006fc((U)l11, &l88, 10, &l70);
        l48 = l70;
      }
    }
  }
  if (-1 < l48) {
    l50[0] = l50[0] & 0xfe;
  }
  if (l11 == 0x7a) {
    l11 = *l88;
    l12 = '\x01';
    pbVar2 = l88 + 1;
  } else {
    pbVar2 = l88;
    if (l11 == 0x6c) {
      l11 = *l88;
      l12 = '\x01';
      pbVar2 = l88 + 1;
      if (l11 == 0x6c) {
        l11 = l88[1];
        l12 = '\x02';
        pbVar2 = l88 + 2;
      }
    }
  }
  l88 = pbVar2;
  if (l11 == 0x69) {
  LAB_88000fe4:
    l40 = 10;
    if (l48 < 0) {
      l48 = 1;
    }
    if (l12 == '\x02') {
      fcn_88000288(*l90, l50);
    } else {
      if (l12 == '\x01') {
        fcn_88000430(*l90, l50);
      } else {
        fcn_880005d4((J) * (I*)l90, l50);
      }
    }
    l90 = l90 + 1;
    fcn_880007d8(l78, l80, l50);
    goto LAB_880012a8;
  }
  if (l11 < 0x6a) {
    if (l11 != 0x58) {
      if (l11 < 0x59) {
        if (l11 == 0) {
          R;
        }
        if (l11 == 0x25) {
          (*(code *)(l80 & 0xfffffffffffffffe))(l78, 0x25);
        }
      } else {
        if (l11 == 99) {
          (*(code *)(l80 & 0xfffffffffffffffe))(l78, (J) * (I*)l90 & 0xff);
          l90 = l90 + 1;
        } else {
          if (l11 == 100) goto LAB_88000fe4;
        }
      }
      goto LAB_880012a8;
    }
  } else {
    if (l11 == 0x73) {
      l18 = l48;
      l38 = *l90;
      l20 = l38;
      while ((l18 != 0 &&
              (pcVar4 = l20 + 1, cVar1 = *l20, l20 = pcVar4, cVar1 != '\0'))) {
        l30 = l30 + 1;
        l18 = l18 + -1;
      }
      l48 = -1;
      l18 = l18 + -1;
      fcn_880007d8(l78, l80, l50);
      l90 = l90 + 1;
      goto LAB_880012a8;
    }
    if (0x73 < l11) {
      if (l11 == 0x75) {
        l40 = 10;
        if (l48 < 0) {
          l48 = 1;
        }
        if (l12 == '\x02') {
          fcn_8800013c(*l90, l50);
        } else {
          if (l12 == '\x01') {
            fcn_880002e4(*l90, l50);
          } else {
            fcn_8800048c((J) * (I*)l90, l50);
          }
        }
        l90 = l90 + 1;
        fcn_880007d8(l78, l80, l50);
      } else {
        if (l11 == 0x78) goto LAB_880010b0;
      }
      goto LAB_880012a8;
    }
    if (l11 == 0x6f) {
      l40 = 8;
      if (l48 < 0) {
        l48 = 1;
      }
      fcn_8800048c((J) * (I*)l90, l50);
      fcn_880007d8(l78, l80, l50);
      l90 = l90 + 1;
      goto LAB_880012a8;
    }
    if (l11 != 0x70) goto LAB_880012a8;
    l50[0] = l50[0] | 2;
    l12 = '\x01';
  }
LAB_880010b0:
  l40 = 0x10;
  l50[0] = l50[0] & 0xfb | (l11 == 0x58) << 2;
  if (l48 < 0) {
    l48 = 1;
  }
  if (l12 == '\x02') {
    fcn_8800013c(*l90, l50);
  } else {
    if (l12 == '\x01') {
      fcn_880002e4(*l90, l50);
    } else {
      fcn_8800048c((J) * (I*)l90, l50);
    }
  }
  l90 = l90 + 1;
  fcn_880007d8(l78, l80, l50);
  goto LAB_880012a8;
}


J tfp_vsnprintf(J a1, J a2, undef8 a3, undef8 a4)
{
  J lVar1;
  U l28;
  J l20;
  U l18;

  if (a2 == 0) {
    lVar1 = 0;
  } else {
    l28 = a2 - 1;
    l18 = 0;
    l20 = a1;
    tfp_format(&l28, &LAB_880013a4, a3, a4);
    if (l18 < l28) {
      *(undef *)(l20 + l18) = 0;
    } else {
      *(undef *)(l20 + l28) = 0;
    }
    lVar1 = (J)(I)l18;
  }
  R lVar1;
}


J tfp_vsprintf(J a1, undef8 a2, undef8 a3)
{
  J l20;
  J l18;

  l18 = 0;
  l20 = a1;
  tfp_format(&l20, &LAB_8800153c, a2, a3);
  *(undef *)(l20 + l18) = 0;
  R(J)(I) l18;
}


J tfp_sprintf(undef8 a1, undef8 a2, undef8 a3, undef8 a4,
            undef8 a5, undef8 a6, undef8 a7, undef8 a8)
{
  I iVar1;
  undef8 l30;
  undef8 uStack40;
  undef8 uStack32;
  undef8 uStack24;
  undef8 uStack16;
  undef8 uStack8;

  l30 = a3;
  uStack40 = a4;
  uStack32 = a5;
  uStack24 = a6;
  uStack16 = a7;
  uStack8 = a8;
  iVar1 = tfp_vsprintf(a1, a2, &l30);
  R(J) iVar1;
}


undef8 printk(undef8 a1, undef8 a2, undef8 a3, undef8 a4,
                undef8 a5, undef8 a6, undef8 a7, undef8 a8)
{
  undef8 uStack56;
  undef8 uStack48;
  undef8 uStack40;
  undef8 uStack32;
  undef8 uStack24;
  undef8 uStack16;
  undef8 uStack8;

  uStack56 = a2;
  uStack48 = a3;
  uStack40 = a4;
  uStack32 = a5;
  uStack24 = a6;
  uStack16 = a7;
  uStack8 = a8;
  tfp_format(_DAT_805fc008, &LAB_88001674, a1, &uStack56);
  R 0;
}


void panic_printf(undef8 *a1, undef8 a2, undef8 a3, J a4)
{
  J lVar1;
  J in_mhartid;
  undef8 *l48;
  undef8 l38[2];
  J l28;
  J l20;
  I l18;
  I l14;

  l38[0] = s_unknown_8800b9d0;
  l48 = a1;
  if (a1 == (undef8 *)0x0) {
    l48 = l38;
  }
  l20 = 0x805fb800;
  l28 = 0x805fbc00;
  if (in_mhartid == 0) {
    tfp_sprintf(0x805fb800, s_Cause_0x_016lx__EPC_0x_016lx__Co_8800b938, a2, a3);
    l14 = 0;
    while (l14 < 0x10) {
      lVar1 = strlen(l20);
      tfp_sprintf(l20 + lVar1, s_reg__02d___s____0x_016lx__reg__0_8800b960,
                (J)(l14 << 1), (&PTR_s_zero_8800bc38)[(J)(l14 << 1) * 2],
                *(undef8 *)(a4 + (J)l14 * 0x10), (J)(l14 * 2 + 1),
                (&PTR_s_zero_8800bc38)[(J)(l14 * 2 + 1) * 2],
                *(undef8 *)(a4 + (J)l14 * 0x10 + 8));
      l14 = l14 + 1;
    }
    lVar1 = strlen(l20);
    tfp_sprintf(l20 + lVar1, s_Reason___s_8800b998, l48);
    uarths_puts(l20);
  } else {
    tfp_sprintf(0x805fbc00, s_Cause_0x_016lx__EPC_0x_016lx__Co_8800b9a8, a2, a3);
    l18 = 0;
    while (l18 < 0x10) {
      lVar1 = strlen(l28);
      tfp_sprintf(l28 + lVar1, s_reg__02d___s____0x_016lx__reg__0_8800b960,
                (J)(l18 << 1), (&PTR_s_zero_8800bc38)[(J)(l18 << 1) * 2],
                *(undef8 *)(a4 + (J)l18 * 0x10), (J)(l18 * 2 + 1),
                (&PTR_s_zero_8800bc38)[(J)(l18 * 2 + 1) * 2],
                *(undef8 *)(a4 + (J)l18 * 0x10 + 8));
      l18 = l18 + 1;
    }
    lVar1 = strlen(l28);
    tfp_sprintf(l28 + lVar1, s_Reason___s_8800b998, l48);
  }
  R;
}


void exit(I a1)
{
  J in_mhartid;

  if (in_mhartid == 0) {
    printk(s_Ieresting__something_s_wrong__b_8800b9d8, (J)a1);
  }
  do {
    // WARNING: Do nothing block with infinite loop
  } while (true);
}


undef8 panic_handler_internal(undef8 a1, uI3 *a2, U a3, J a4, I a5,
                                undef4 a6, uI a7)
{
  I iVar1;
  uI uVar2;
  uI3 *puVar3;
  J lVar4;
  U extraout_a1;
  C bVar5;
  C bVar6;
  C bVar7;
  uI uStack88;
  I iStack68;
  uI uStack48;

  panic_printf(s_fuck_the_chip_is_dead__8800ba30, a1);
  puVar3 = (uI3 *)exit(0x29a);
  bVar5 = (C)extraout_a1;
  bVar6 = (C)a3;
  uStack88 = 0;
  if ((a5 == 0) || (a5 == 1)) {
    uStack48 = uStack48 + 0xf & 0xfffffff0;
  }
  _DAT_50450028 = _DAT_50450028 | 1;
  iStack68 = 0;
  while (iStack68 < (I)(uI)(bVar5 >> 2)) {
    *(undef4 *)((J)iStack68 * 4 + 0x50450000) =
        *(undef4 *)((J)puVar3 + (J)(I)((uI)bVar5 + iStack68 * -4) + -4);
    iStack68 = iStack68 + 1;
  }
  bVar7 = bVar5 & 3;
  if ((extraout_a1 & 3) != 0) {
    if (bVar7 == 2) {
      uStack88 = (uI) * (ushort *)puVar3;
    } else {
      if (bVar7 == 3) {
        uStack88 = (uI)*puVar3;
      } else {
        if (bVar7 == 1) {
          uStack88 = (uI) * (C*)puVar3;
        }
      }
    }
    *(uI*)((J)(I)(uI)(bVar5 >> 2) * 4 + 0x50450000) = uStack88;
  }
  iStack68 = 0;
  while (iStack68 < (I)(uI)(bVar6 >> 2)) {
    *(undef4 *)(((J)iStack68 + 4) * 4 + 0x50450008) =
        *(undef4 *)((J)a2 + (J)(I)((uI)bVar6 + iStack68 * -4) + -4);
    iStack68 = iStack68 + 1;
  }
  bVar5 = bVar6 & 3;
  if ((a3 & 3) != 0) {
    if (bVar5 == 2) {
      uStack88 = uStack88 & 0xffff0000 | (uI) * (ushort *)a2;
    } else {
      if (bVar5 == 3) {
        uStack88 = (uI)*a2;
      } else {
        if (bVar5 == 1) {
          uStack88 = uStack88 & 0xffffff00 | (uI) * (C*)a2;
        }
      }
    }
    *(uI*)(((J)(I)(uI)(bVar6 >> 2) + 4) * 4 + 0x50450008) = uStack88;
  }
  _DAT_50450034 = a7 - 1;
  _DAT_5045003c = uStack48 - 1;
  _DAT_50450064 = _DAT_50450064 | 1;
  _DAT_50450010 = a6;
  _DAT_50450014 = a5;
  if (a5 == 3) {
    iStack68 = 0;
    while (iStack68 < (I)(a7 >> 2)) {
      iVar1 = *(I*)(a4 + (iStack68 << 2));
      do {
        lVar4 = aes_get_data_in_flag();
      } while (lVar4 == 0);
      aes_write_aad((J)iVar1);
      iStack68 = iStack68 + 1;
    }
    uVar2 = a7 & 0xfffffffc;
    a7 = a7 & 3;
    if (a7 != 0) {
      if (a7 == 2) {
        uStack88._0_2_ = CONCAT11(*(undef *)(a4 + (J)(I)uVar2 + 1),
                                  *(undef *)(a4 + (I)uVar2));
        uStack88 = uStack88 & 0xffff0000 | (uI)(ushort)uStack88;
      } else {
        if (a7 == 3) {
          uStack88._0_2_ = CONCAT11(*(undef *)(a4 + (J)(I)uVar2 + 1),
                                    *(undef *)(a4 + (I)uVar2));
          uStack88._0_3_ =
              CONCAT12(*(undef *)(a4 + (J)(I)uVar2 + 2), (ushort)uStack88);
          uStack88 = (uI)(uI3)uStack88;
        } else {
          if (a7 != 1) {
            R 0;
          }
          uStack88 = uStack88 & 0xffffff00 | (uI) * (C*)(a4 + (I)uVar2);
        }
      }
      do {
        lVar4 = aes_get_data_in_flag();
      } while (lVar4 == 0);
      aes_write_aad((J)(I)uStack88);
    }
  }
  R 1;
}


undef8 aes_init(uI3 *a1, C a2, uI3 *a3, C a4, J a5, I a6, undef4 a7,
                    uI a8)
{
  I iVar1;
  uI uVar2;
  J lVar3;
  C bVar4;
  uI in_stack_00000000;
  uI l28;
  I l14;

  l28 = 0;
  if ((a6 == 0) || (a6 == 1)) {
    in_stack_00000000 = in_stack_00000000 + 0xf & 0xfffffff0;
  }
  _DAT_50450028 = _DAT_50450028 | 1;
  l14 = 0;
  while (l14 < (I)(uI)(a2 >> 2)) {
    *(undef4 *)((J)l14 * 4 + 0x50450000) =
        *(undef4 *)((J)a1 + (J)(I)((uI)a2 + l14 * -4) + -4);
    l14 = l14 + 1;
  }
  bVar4 = a2 & 3;
  if ((a2 & 3) != 0) {
    if (bVar4 == 2) {
      l28 = (uI) * (ushort *)a1;
    } else {
      if (bVar4 == 3) {
        l28 = (uI)*a1;
      } else {
        if (bVar4 == 1) {
          l28 = (uI) * (C*)a1;
        }
      }
    }
    *(uI*)((J)(I)(uI)(a2 >> 2) * 4 + 0x50450000) = l28;
  }
  l14 = 0;
  while (l14 < (I)(uI)(a4 >> 2)) {
    *(undef4 *)(((J)l14 + 4) * 4 + 0x50450008) =
        *(undef4 *)((J)a3 + (J)(I)((uI)a4 + l14 * -4) + -4);
    l14 = l14 + 1;
  }
  bVar4 = a4 & 3;
  if ((a4 & 3) != 0) {
    if (bVar4 == 2) {
      l28 = l28 & 0xffff0000 | (uI) * (ushort *)a3;
    } else {
      if (bVar4 == 3) {
        l28 = (uI)*a3;
      } else {
        if (bVar4 == 1) {
          l28 = l28 & 0xffffff00 | (uI) * (C*)a3;
        }
      }
    }
    *(uI*)(((J)(I)(uI)(a4 >> 2) + 4) * 4 + 0x50450008) = l28;
  }
  _DAT_50450034 = a8 - 1;
  _DAT_5045003c = in_stack_00000000 - 1;
  _DAT_50450064 = _DAT_50450064 | 1;
  _DAT_50450010 = a7;
  _DAT_50450014 = a6;
  if (a6 == 3) {
    l14 = 0;
    while (l14 < (I)(a8 >> 2)) {
      iVar1 = *(I*)(a5 + (l14 << 2));
      do {
        lVar3 = aes_get_data_in_flag();
      } while (lVar3 == 0);
      aes_write_aad((J)iVar1);
      l14 = l14 + 1;
    }
    uVar2 = a8 & 0xfffffffc;
    a8 = a8 & 3;
    if (a8 != 0) {
      if (a8 == 2) {
        l28._0_2_ = CONCAT11(*(undef *)(a5 + (J)(I)uVar2 + 1),
                             *(undef *)(a5 + (I)uVar2));
        l28 = l28 & 0xffff0000 | (uI)(ushort)l28;
      } else {
        if (a8 == 3) {
          l28._0_2_ = CONCAT11(*(undef *)(a5 + (J)(I)uVar2 + 1),
                               *(undef *)(a5 + (I)uVar2));
          l28._0_3_ =
              CONCAT12(*(undef *)(a5 + (J)(I)uVar2 + 2), (ushort)l28);
          l28 = (uI)(uI3)l28;
        } else {
          if (a8 != 1) {
            R 0;
          }
          l28 = l28 & 0xffffff00 | (uI) * (C*)(a5 + (I)uVar2);
        }
      }
      do {
        lVar3 = aes_get_data_in_flag();
      } while (lVar3 == 0);
      aes_write_aad((J)(I)l28);
    }
  }
  R 1;
}


undef8 aes_write_aad(undef4 a1)
{
  _DAT_50450044 = a1;
  R 0;
}


undef8 aes_write_text(undef4 a1)
{
  _DAT_50450040 = a1;
  R 0;
}


undef8 gcm_write_tag(undef4 *a1)
{
  _DAT_50450050 = a1[3];
  _DAT_50450054 = a1[2];
  _DAT_50450058 = a1[1];
  _DAT_5045005c = *a1;
  R 0;
}


J aes_get_data_in_flag(void)
{
  R(J) _DAT_5045004c;
}


J fcn_8800208c_aes(void)
{
  R(J) _DAT_50450068;
}


J gcm_get_tag_in_flag(void)
{
  R(J) _DAT_5045006c;
}


J aes_read_out_data(void)
{
  R(J) _DAT_50450060;
}


J gcm_get_tag_chk(void)
{
  R(J) _DAT_50450048;
}


undef8 FUN_88002138(undef *a1)
{
  undef4 uVar1;

  uVar1 = _DAT_50450080;
  *a1 = (C)((uI)_DAT_50450080 >> 0x18);
  a1[1] = (C)((uI)uVar1 >> 0x10);
  a1[2] = (C)((uI)uVar1 >> 8);
  a1[3] = (C)uVar1;
  uVar1 = _DAT_5045007c;
  a1[4] = (C)((uI)_DAT_5045007c >> 0x18);
  a1[5] = (C)((uI)uVar1 >> 0x10);
  a1[6] = (C)((uI)uVar1 >> 8);
  a1[7] = (C)uVar1;
  uVar1 = _DAT_50450078;
  a1[8] = (C)((uI)_DAT_50450078 >> 0x18);
  a1[9] = (C)((uI)uVar1 >> 0x10);
  a1[10] = (C)((uI)uVar1 >> 8);
  a1[0xb] = (C)uVar1;
  uVar1 = _DAT_50450074;
  a1[0xc] = (C)((uI)_DAT_50450074 >> 0x18);
  a1[0xd] = (C)((uI)uVar1 >> 0x10);
  a1[0xe] = (C)((uI)uVar1 >> 8);
  a1[0xf] = (C)uVar1;
  R 1;
}



undef8 gcm_clear_chk_tag(void)
{
  _DAT_50450070 = 0;
  R 0;
}


undef8 aes_process(J a1, J a2, uI a3, I a4)
{
  I iVar1;
  uI uVar2;
  undef4 uVar3;
  J lVar4;
  uI uVar5;
  uI uVar6;
  undef4 l40;
  I l28;
  uI l24;

  uVar5 = a3 + 0xf & 0xfffffff0;
  l24 = a3 >> 2;
  l28 = 0;
  while (l28 < (I)l24) {
    iVar1 = *(I*)(a1 + (l28 << 2));
    do {
      lVar4 = aes_get_data_in_flag();
    } while (lVar4 == 0);
    aes_write_text((J)iVar1);
    l28 = l28 + 1;
  }
  uVar2 = a3 & 0xfffffffc;
  uVar6 = a3 & 3;
  if (uVar6 != 0) {
    if (uVar6 == 2) {
      l40._0_2_ = CONCAT11(*(undef *)(a1 + (J)(I)uVar2 + 1),
                           *(undef *)(a1 + (I)uVar2));
      l40 = (uI)(ushort)l40;
    } else {
      if (uVar6 == 3) {
        l40._0_2_ = CONCAT11(*(undef *)(a1 + (J)(I)uVar2 + 1),
                             *(undef *)(a1 + (I)uVar2));
        l40._0_3_ = CONCAT12(*(undef *)(a1 + (J)(I)uVar2 + 2), (ushort)l40);
        l40 = (uI)(uI3)l40;
      } else {
        if (uVar6 != 1) {
          R 0;
        }
        l40 = (uI) * (C*)(a1 + (I)uVar2);
      }
    }
    do {
      lVar4 = aes_get_data_in_flag();
    } while (lVar4 == 0);
    aes_write_text((J)(I)l40);
  }
  if ((a4 == 0) || (a4 == 1)) {
    l28 = 0;
    while (l28 < (I)(uVar5 - a3 >> 2)) {
      do {
        lVar4 = aes_get_data_in_flag();
      } while (lVar4 == 0);
      aes_write_text(0);
      l28 = l28 + 1;
    }
    l24 = (I)(((uI)((I)(a3 + 0xf) >> 0x1f) >> 0x1e) + uVar5) >> 2;
  }
  l28 = 0;
  while (l28 < (I)l24) {
    do {
      lVar4 = fcn_8800208c_aes();
    } while (lVar4 == 0);
    uVar3 = aes_read_out_data();
    *(undef4 *)(a2 + (l28 << 2)) = uVar3;
    l28 = l28 + 1;
  }
  if ((a4 == 3) && (uVar6 != 0)) {
    do {
      lVar4 = fcn_8800208c_aes();
    } while (lVar4 == 0);
    uVar3 = aes_read_out_data();
    l40._0_1_ = (undef)uVar3;
    l40._1_1_ = (undef)((uI)uVar3 >> 8);
    if (uVar6 == 2) {
      *(undef *)(a2 + (I)(l24 << 2)) = (undef)l40;
      *(undef *)(a2 + (J)(l28 << 2) + 1) = l40._1_1_;
    } else {
      if (uVar6 == 3) {
        *(undef *)(a2 + (I)(l24 << 2)) = (undef)l40;
        *(undef *)(a2 + (J)(l28 << 2) + 1) = l40._1_1_;
        l40._2_1_ = (undef)((uI)uVar3 >> 0x10);
        *(undef *)(a2 + (J)(l28 << 2) + 2) = l40._2_1_;
      } else {
        if (uVar6 != 1) {
          R 0;
        }
        *(undef *)(a2 + (I)(l24 << 2)) = (undef)l40;
      }
    }
  }
  R 1;
}


undef8 aes_process_outer(J a1, J a2, uI a3, I a4)
{
  uI l14;

  l14 = 0;
  if (0x4f < a3) {
    l14 = 0;
    while (l14 < a3 / 0x50) {
      aes_process(a1 + ((J)(I)(l14 * 0x50) & 0xffffffffU),
                  a2 + ((J)(I)(l14 * 0x50) & 0xffffffffU), 0x50, (J)a4);
      l14 = l14 + 1;
    }
  }
  if (a3 % 0x50 != 0) {
    aes_process(a1 + ((J)(I)(l14 * 0x50) & 0xffffffffU),
                a2 + ((J)(I)(l14 * 0x50) & 0xffffffffU), (J)(I)(a3 % 0x50),
                (J)a4);
  }
  R 1;
}



undef8 clint_ipi_send(U a1)
{
  uI*puVar1;
  undef8 uVar2;

  if (a1 < 2) {
    puVar1 = (uI*)(a1 * 4 + 0x2000000);
    *puVar1 = *puVar1 | 1;
    uVar2 = 0;
  } else {
    uVar2 = 0xffffffffffffffff;
  }
  R uVar2;
}


undef8 fpioa_init(void)
{
  uI uVar1;
  J lVar2;
  undef8 l58;
  undef8 l50;
  undef8 l48;
  undef8 l40;
  undef8 l38;
  undef8 l30;
  undef8 l28;
  undef8 l20;
  I l14;

  l14 = 0;
  sysctl_clock_enable(0x1d);
  l58 = 0;
  l50 = 0;
  l48 = 0;
  l40 = 0;
  l38 = 0;
  l30 = 0;
  l28 = 0;
  l20 = 0;
  l14 = 0;
  while (l14 < 0x100) {
    uVar1 = l14 >> 0x1f;
    lVar2 = (J)((I)((uVar1 >> 0x1b) + l14) >> 5);
    *(uI*)((J)&l58 + lVar2 * 4) =
        *(uI*)((J)&l58 + lVar2 * 4) |
        (*(uI*)(&DAT_8800b4c0 + (J)l14 * 4) >> 0x18 & 1)
            << ((J)(I)((l14 + (uVar1 >> 0x1b) & 0x1f) - (uVar1 >> 0x1b)) &
                0x1fU);
    lVar2 = (J)((I)((uVar1 >> 0x1b) + l14) >> 5);
    *(uI*)((J)&l58 + (lVar2 + 8) * 4) =
        *(uI*)((J)&l58 + (lVar2 + 8) * 4) |
        (*(uI*)(&DAT_8800b4c0 + (J)l14 * 4) >> 0x19 & 1)
            << ((J)(I)((l14 + (uVar1 >> 0x1b) & 0x1f) - (uVar1 >> 0x1b)) &
                0x1fU);
    l14 = l14 + 1;
  }
  l14 = 0;
  while (l14 < 8) {
    *(undef4 *)(((J)l14 + 0x38) * 4 + 0x502b0000) =
        *(undef4 *)((J)&l58 + ((J)l14 + 8) * 4);
    *(undef4 *)(((J)l14 + 0x30) * 4 + 0x502b0000) =
        *(undef4 *)((J)&l58 + (J)l14 * 4);
    l14 = l14 + 1;
  }
  R 0;
}


undef8 fpioa_set_function_raw(I a1, uI a2)
{
  undef8 uVar1;

  if (((a1 < 0) || (0x2f < a1)) || (0xff < a2)) {
    uVar1 = 0xffffffffffffffff;
  } else {
    *(uI*)((J)a1 * 4 + 0x502b0000) =
        (uI)(C)(&DAT_8800b4c0)[(U)a2 * 4] |
        (*(uI*)(&DAT_8800b4c0 + (U)a2 * 4) >> 8 & 0xf) << 8 |
        (*(uI*)(&DAT_8800b4c0 + (U)a2 * 4) >> 0xc & 1) << 0xc |
        (*(uI*)(&DAT_8800b4c0 + (U)a2 * 4) >> 0xd & 1) << 0xd |
        (*(uI*)(&DAT_8800b4c0 + (U)a2 * 4) >> 0xe & 1) << 0xe |
        (*(uI*)(&DAT_8800b4c0 + (U)a2 * 4) >> 0xf & 1) << 0xf |
        (*(uI*)(&DAT_8800b4c0 + (U)a2 * 4) >> 0x10 & 1) << 0x10 |
        (*(uI*)(&DAT_8800b4c0 + (U)a2 * 4) >> 0x11 & 1) << 0x11 |
        (*(uI*)(&DAT_8800b4c0 + (U)a2 * 4) >> 0x12 & 1) << 0x12 |
        (*(uI*)(&DAT_8800b4c0 + (U)a2 * 4) >> 0x13 & 1) << 0x13 |
        (*(uI*)(&DAT_8800b4c0 + (U)a2 * 4) >> 0x14 & 1) << 0x14 |
        (*(uI*)(&DAT_8800b4c0 + (U)a2 * 4) >> 0x15 & 1) << 0x15 |
        (*(uI*)(&DAT_8800b4c0 + (U)a2 * 4) >> 0x16 & 1) << 0x16 |
        (*(uI*)(&DAT_8800b4c0 + (U)a2 * 4) >> 0x17 & 1) << 0x17;
    uVar1 = 0;
  }
  R uVar1;
}


undef8 fpioa_set_function(uI a1, uI a2)
{
  undef8 uVar1;
  C l11;

  if ((((I)a1 < 0) || (0x2f < (I)a1)) || (0xff < a2)) {
    uVar1 = 0xffffffffffffffff;
  } else {
    if (a2 == 0x78) {
      fpioa_set_function_raw((J)(I)a1, 0x78);
      uVar1 = 0;
    } else {
      l11 = 0;
      while (l11 < 0x30) {
        if ((a2 == (*(uI*)((J)(I)(uI)l11 * 4 + 0x502b0000) & 0xff)) &&
            (a1 != (uI)l11)) {
          fpioa_set_function_raw((J)(I)(uI)l11, 0x78);
        }
        l11 = l11 + 1;
      }
      fpioa_set_function_raw((J)(I)a1, (J)(I)a2);
      uVar1 = 0;
    }
  }
  R uVar1;
}


void otp_reset(C a1)
{
  sysctl_clock_enable(0x24);
  _DAT_5042000c = 0;
  _DAT_50420028 = 0;
  _DAT_50420044 = (uI)a1;
  _DAT_5042005c = 0;
  _DAT_50420078 = 0;
  _DAT_5042007c = 1;
  _DAT_50420080 = 1;
  _DAT_50420084 = 0;
  _DAT_50420088 = 0;
  _DAT_5042008c = 0;
  _DAT_50420090 = 0;
  _DAT_50420094 = 0;
  _DAT_50420098 = 0;
  _DAT_5042009c = 0;
  _DAT_504200a0 = 1;
  _DAT_504200a4 = 0;
  _DAT_504200a8 = 0;
  R;
}


void otp_set_0c(void)
{
  _DAT_5042000c = 1;
  R;
}


void otp_clear_0c(void)
{
  _DAT_5042000c = 0;
  R;
}


undef8 otp_check_fuse_a_bit(uI a1)
{
  undef8 uVar1;

  if ((a1 & _DAT_50420068) == 0) {
    uVar1 = 0x10;
  } else {
    uVar1 = 0xf;
  }
  R uVar1;
}


undef8 otp_clear_state18(void)
{
  uI l14;

  l14 = 0;
  do {
    if (_DAT_50420048 != 0) {
      if (_DAT_50420064 != 0) {
        _DAT_50420000 = 0;
        _DAT_50420004 = 0x30;
        _DAT_50420008 = 2;
        _DAT_50420018 = 0;
        _DAT_50420048 = 0;
        _DAT_50420058 = 1;
        R 5;
      }
      _DAT_50420000 = 0;
      _DAT_50420004 = 0x30;
      _DAT_50420008 = 2;
      _DAT_50420018 = 0;
      _DAT_50420048 = 0;
      _DAT_50420058 = 1;
      R 0;
    }
    l14 = l14 + 1;
  } while (l14 < 0xffffff);
  _DAT_50420000 = 0;
  _DAT_50420004 = 0x30;
  _DAT_50420008 = 2;
  _DAT_50420018 = 0;
  _DAT_50420058 = 1;
  R 1;
}


undef8 otp_set_state18(void)
{
  uI l14;

  l14 = 0;
  do {
    if (_DAT_50420048 != 0) {
      _DAT_50420000 = 0;
      _DAT_50420004 = 0x30;
      _DAT_50420008 = 2;
      _DAT_50420018 = 1;
      _DAT_50420048 = 0;
      _DAT_50420058 = 1;
      R 0;
    }
    l14 = l14 + 1;
  } while (l14 < 0xffffff);
  _DAT_50420000 = 0;
  _DAT_50420004 = 0x30;
  _DAT_50420008 = 2;
  _DAT_50420018 = 1;
  _DAT_50420058 = 1;
  R 1;
}


J fcn_880042c0_otp(undef8 a1, C*a2, undef8 a3)
{
  I iVar1;
  C*l30;
  I l28;
  I l24;
  uI l18;
  C l12;
  C l11;

  _DAT_50420058 = 0;
  _DAT_50420008 = 1;
  _DAT_50420060 = 0;
  _DAT_50420000 = 0;
  l12 = '\0';
  l24 = (I)a1 << 3;
  l11 = *a2;
  l30 = a2 + 1;
  l28 = (I)a3 << 3;
  while (true) {
    _DAT_50420038 = 0;
    iVar1 = l28 + -1;
    if (l28 == 0) break;
    if ((iVar1 == 0) || ((l11 & 1) != 0)) {
      l18 = 0;
      do {
        if (_DAT_5042003c != 0) {
          if (iVar1 == 0) {
            _DAT_5042002c = (uI)l11 & 1;
            _DAT_50420020 = 1;
          } else {
            _DAT_5042002c = 1;
          }
          l18 = 0;
          do {
            l18 = l18 + 1;
          } while (l18 < 0xffffff);
          _DAT_50420000 = 0;
          _DAT_50420008 = 1;
          _DAT_50420010 = 1;
          _DAT_50420030 = l24;
          _DAT_50420038 = 0;
          _DAT_50420058 = 0;
          _DAT_50420060 = 0;
          R 1;
        }
        l18 = l18 + 1;
      } while (l18 < 0xffffff);
      _DAT_50420000 = 0;
      _DAT_50420008 = 1;
      _DAT_50420038 = 0;
      _DAT_50420058 = 0;
      _DAT_50420060 = 0;
      R 1;
    }
    l11 = l11 >> 1;
    l24 = l24 + 1;
    l12 = l12 + '\x01';
    l28 = iVar1;
    if (l12 == '\b') {
      l12 = '\0';
      l11 = *l30;
      l30 = l30 + 1;
    }
  }
  iVar1 = otp_clear_state18();
  if (iVar1 != 0) {
    R(J) iVar1;
  }
  iVar1 = otp_set_state18();
  if (iVar1 != 0) {
    R(J) iVar1;
  }
  R 0;
}


undef8 otp_read_inner(I a1, undef *a2, I a3)
{
  undef *l30;
  I l28;
  I l24;
  uI l14;

  _DAT_50420058 = 0;
  _DAT_50420008 = 0;
  _DAT_50420060 = 0;
  _DAT_50420000 = 0;
  l24 = a1 << 3;
  l30 = a2;
  l28 = a3;
  while (true) {
    if (l28 == 0) {
      R 0;
    }
    l14 = 0;
    while (_DAT_5042003c == 0) {
      l14 = l14 + 1;
      if (0xfffffe < l14) {
        R 1;
      }
    }
    if (l28 + -1 == 0) {
      _DAT_50420020 = 1;
    }
    _DAT_50420030 = l24;
    l14 = 0;
    while (_DAT_504200ac == 0) {
      l14 = l14 + 1;
      if (0xfffffe < l14) {
        R 1;
      }
    }
    if (_DAT_50420060 == 1) break;
    *l30 = (C)_DAT_50420024;
    l24 = l24 + 8;
    l30 = l30 + 1;
    l28 = l28 + -1;
  }
  R 2;
}


J otp_read(uI a1, undef8 a2, uI a3)
{
  J lVar1;
  I l14;

  if (a1 < 0x3dd0) {
    if (0x3dd0 - a1 < a3) {
      a3 = 0x3dd0 - a1;
    }
    l14 = otp_read_inner((J)(I)a1, a2, (J)(I)a3);
    if (l14 == 2) {
      l14 = 10;
    }
    lVar1 = (J)l14;
  } else {
    lVar1 = 2;
  }
  R lVar1;
}


J fcn_88004bd4_otp(uI a1)
{
  I iVar1;
  J lVar2;
  undef l15[5];

  if (a1 < 0x40) {
    _DAT_5042005c = 1;
    l15[0] = (undef)(3 << (J)(I)((a1 & 3) << 1));
    iVar1 = fcn_880042c0_otp((J)(I)((a1 >> 2) + 0x3fe0), l15, 1);
    _DAT_5042005c = 0;
    lVar2 = (J)iVar1;
  } else {
    lVar2 = 9;
  }
  R lVar2;
}


undef8 otp_check_fuse_b_bit(uI a1)
{
  if (a1 < 0x20) {
    if ((_DAT_504200bc & 1 << ((J)(I)a1 & 0x1fU)) != 0) {
      R 0xe;
    }
  } else {
    if (0x3f < a1) {
      R 9;
    }
    if ((_DAT_504200b8 & 1 << ((J)(I)(a1 - 0x20) & 0x1fU)) != 0) {
      R 0xe;
    }
  }
  R 0xd;
}


J fcn_88004e9c_otp_read(uI a1)
{
  I iVar1;
  J lVar2;
  undef uStack17;

  if (a1 < 0x40) {
    iVar1 = otp_read_inner((J)(I)((a1 >> 2) + 0x3fe0), &uStack17, 1);
    lVar2 = (J)iVar1;
  } else {
    lVar2 = 9;
  }
  R lVar2;
}


U fcn_880052a0(undef8 a1)
{
  uI uVar1;
  uI uVar2;

  uVar1 = (uI)((U)a1 >> 0x20);
  uVar2 = (uI)a1;
  R((J)(I)(uVar2 >> 8 | uVar2 << 0x18) & 0xff00ff00U |
    (J)(I)(uVar2 << 8 | uVar2 >> 0x18) & 0xff00ffU)
          << 0x20 |
      (J)(I)(uVar1 >> 8 | uVar1 << 0x18) & 0xff00ff00U |
      (J)(I)(uVar1 << 8 | uVar1 >> 0x18) & 0xff00ffU;
}


undef8 sha256_init(C a1, C a2, I a3, undef8 *a4)
{
  sysctl_clock_enable(0x23);
  sysctl_reset(0x1b);
  _DAT_502c0028 = a3 + 0x40U >> 6;
  if (a1 == '\0') {
    _DAT_502c0034 = 0;
  } else {
    _DAT_502c0034 = _DAT_502c0034 | 1;
  }
  if (a2 != '\0') {
    _DAT_502c0030 = _DAT_502c0030 | 1;
  }
  _DAT_502c002c = _DAT_502c002c | 0x10001;
  *a4 = 0;
  *(undef4 *)(a4 + 1) = 0x6a09e667;
  *(undef4 *)((J)a4 + 0xc) = 0xbb67ae85;
  *(undef4 *)(a4 + 2) = 0x3c6ef372;
  *(undef4 *)((J)a4 + 0x14) = 0xa54ff53a;
  *(undef4 *)(a4 + 3) = 0x510e527f;
  *(undef4 *)((J)a4 + 0x1c) = 0x9b05688c;
  *(undef4 *)(a4 + 4) = 0x1f83d9ab;
  *(undef4 *)((J)a4 + 0x24) = 0x5be0cd19;
  *(undef4 *)(a4 + 5) = 0;
  R 1;
}


void sha256_update(J*a1, J a2, uI a3)
{
  uI l44;
  uI l20;
  uI l1c;
  J l18;

  l44 = a3;
  l18 = a2;
  while (l44 != 0) {
    l1c = 0x40 - *(I*)(a1 + 5);
    if (l44 < l1c) {
      l1c = l44;
    }
    memcpy((J)a1 + ((J) * (I*)(a1 + 5) & 0xffffffffU) + 0x2c, l18, (U)l1c);
    *a1 = *a1 + (U)l1c * 8;
    *(uI*)(a1 + 5) = l1c + *(I*)(a1 + 5);
    l18 = l18 + (U)l1c;
    l44 = l44 - l1c;
    if (*(I*)(a1 + 5) == 0x40) {
      l20 = 0;
      while (l20 < 0x10) {
        do {
        } while ((_DAT_502c0034 & 0x100) != 0);
        _DAT_502c0020 = *(undef4 *)((J)a1 + ((U)l20 + 8) * 4 + 0xc);
        l20 = l20 + 1;
      }
      *(undef4 *)(a1 + 5) = 0;
    }
  }
  R;
}


void sha256_final(undef8 *a1, undef4 *a2)
{
  undef4 *l30;
  undef8 l20;
  I l18;
  uI l14;

  l14 = -*(I*)(a1 + 5) + 0x78;
  if (0x40 < l14) {
    l14 = -*(I*)(a1 + 5) + 0x38;
  }
  l20 = fcn_880052a0(*a1);
  sha256_update(a1, &DAT_8800be38, (J)(I)l14);
  sha256_update(a1, &l20, 8);
  do {
  } while ((_DAT_502c002c & 1) == 0);
  if (a2 != (undef4 *)0x0) {
    l18 = 0;
    l30 = a2;
    while (l18 < 8) {
      *l30 = *(undef4 *)((J)(7 - l18) * 4 + 0x502c0000);
      l30 = l30 + 1;
      l18 = l18 + 1;
    }
  }
  R;
}


undef8 sysctl_get_freq(void)
{
  R 26000000;
}


void sysctl_reset_ctl(undef4 a1, C a2)
{
  switch (a1) {
    case 0:
      _DAT_50440030 = _DAT_50440030 & 0xfffffffe | (uI)a2 & 1;
      break;
    case 1:
      _DAT_50440034 = _DAT_50440034 & 0xfffffffe | (uI)a2 & 1;
      break;
    case 2:
      _DAT_50440034 = _DAT_50440034 & 0xfffffffd | ((uI)a2 & 1) << 1;
      break;
    case 3:
      _DAT_50440034 = _DAT_50440034 & 0xfffffffb | ((uI)a2 & 1) << 2;
      break;
    case 4:
      _DAT_50440034 = _DAT_50440034 & 0xfffffff7 | ((uI)a2 & 1) << 3;
      break;
    case 5:
      _DAT_50440034 = _DAT_50440034 & 0xffffffef | ((uI)a2 & 1) << 4;
      break;
    case 6:
      _DAT_50440034 = _DAT_50440034 & 0xffffffdf | ((uI)a2 & 1) << 5;
      break;
    case 7:
      _DAT_50440034 = _DAT_50440034 & 0xffffffbf | ((uI)a2 & 1) << 6;
      break;
    case 8:
      _DAT_50440034 = _DAT_50440034 & 0xffffff7f | ((uI)a2 & 1) << 7;
      break;
    case 9:
      _DAT_50440034 = _DAT_50440034 & 0xfffffeff | ((uI)a2 & 1) << 8;
      break;
    case 10:
      _DAT_50440034 = _DAT_50440034 & 0xfffffdff | ((uI)a2 & 1) << 9;
      break;
    case 0xb:
      _DAT_50440034 = _DAT_50440034 & 0xfffffbff | ((uI)a2 & 1) << 10;
      break;
    case 0xc:
      _DAT_50440034 = _DAT_50440034 & 0xfffff7ff | ((uI)a2 & 1) << 0xb;
      break;
    case 0xd:
      _DAT_50440034 = _DAT_50440034 & 0xffffefff | ((uI)a2 & 1) << 0xc;
      break;
    case 0xe:
      _DAT_50440034 = _DAT_50440034 & 0xffffdfff | ((uI)a2 & 1) << 0xd;
      break;
    case 0xf:
      _DAT_50440034 = _DAT_50440034 & 0xffffbfff | ((uI)a2 & 1) << 0xe;
      break;
    case 0x10:
      _DAT_50440034 = _DAT_50440034 & 0xffff7fff | ((uI)a2 & 1) << 0xf;
      break;
    case 0x11:
      _DAT_50440034 = _DAT_50440034 & 0xfffeffff | ((uI)a2 & 1) << 0x10;
      break;
    case 0x12:
      _DAT_50440034 = _DAT_50440034 & 0xfffdffff | ((uI)a2 & 1) << 0x11;
      break;
    case 0x13:
      _DAT_50440034 = _DAT_50440034 & 0xfffbffff | ((uI)a2 & 1) << 0x12;
      break;
    case 0x14:
      _DAT_50440034 = _DAT_50440034 & 0xfff7ffff | ((uI)a2 & 1) << 0x13;
      break;
    case 0x15:
      _DAT_50440034 = _DAT_50440034 & 0xffefffff | ((uI)a2 & 1) << 0x14;
      break;
    case 0x16:
      _DAT_50440034 = _DAT_50440034 & 0xffdfffff | ((uI)a2 & 1) << 0x15;
      break;
    case 0x17:
      _DAT_50440034 = _DAT_50440034 & 0xffbfffff | ((uI)a2 & 1) << 0x16;
      break;
    case 0x18:
      _DAT_50440034 = _DAT_50440034 & 0xff7fffff | ((uI)a2 & 1) << 0x17;
      break;
    case 0x19:
      _DAT_50440034 = _DAT_50440034 & 0xfeffffff | ((uI)a2 & 1) << 0x18;
      break;
    case 0x1a:
      _DAT_50440034 = _DAT_50440034 & 0xfdffffff | ((uI)a2 & 1) << 0x19;
      break;
    case 0x1b:
      _DAT_50440034 = _DAT_50440034 & 0xfbffffff | ((uI)a2 & 1) << 0x1a;
      break;
    case 0x1c:
      _DAT_50440034 = _DAT_50440034 & 0xdfffffff | ((uI)a2 & 1) << 0x1d;
      break;
    case 0x1d:
      _DAT_50440034 = _DAT_50440034 & 0xbfffffff | ((uI)a2 & 1) << 0x1e;
      break;
    case 0x1e:
      _DAT_50440034 = _DAT_50440034 & 0x7fffffff | (uI)a2 << 0x1f;
  }
  R;
}


void sysctl_reset(I a1)
{
  sysctl_reset_ctl((J)a1, 1);
  sysctl_reset_ctl((J)a1, 0);
  R;
}


undef8 sysctl_clock_bus_en(undef4 a1, C a2)
{
  if (a2 != 0) {
    switch (a1) {
      case 0xe:
      case 0x11:
      case 0x13:
      case 0x14:
      case 0x15:
      case 0x16:
      case 0x17:
      case 0x18:
      case 0x19:
      case 0x1a:
      case 0x1b:
      case 0x1d:
      case 0x1e:
      case 0x1f:
      case 0x20:
      case 0x23:
        _DAT_50440028 = _DAT_50440028 & 0xfffffff7 | ((uI)a2 & 1) << 3;
        break;
      case 0xf:
      case 0x10:
        _DAT_50440028 = _DAT_50440028 & 0xffffffdf | ((uI)a2 & 1) << 5;
        break;
      case 0x1c:
      case 0x21:
      case 0x22:
      case 0x24:
      case 0x25:
        _DAT_50440028 = _DAT_50440028 & 0xffffffef | ((uI)a2 & 1) << 4;
    }
  }
  R 0;
}


undef8 sysctl_clock_device_en(undef4 a1, C a2)
{
  switch (a1) {
    case 0:
      _DAT_50440008 = _DAT_50440008 & 0xfdffffff | ((uI)a2 & 1) << 0x19;
      break;
    case 1:
      _DAT_5044000c = _DAT_5044000c & 0xfdffffff | ((uI)a2 & 1) << 0x19;
      break;
    case 2:
      _DAT_50440010 = _DAT_50440010 & 0xfdffffff | ((uI)a2 & 1) << 0x19;
      break;
    case 3:
      _DAT_50440028 = _DAT_50440028 & 0xfffffffe | (uI)a2 & 1;
      break;
    case 4:
      _DAT_50440028 = _DAT_50440028 & 0xfffffffd | ((uI)a2 & 1) << 1;
      break;
    case 5:
      _DAT_50440028 = _DAT_50440028 & 0xfffffffb | ((uI)a2 & 1) << 2;
      break;
    case 6:
      _DAT_50440028 = _DAT_50440028 & 0xfffffff7 | ((uI)a2 & 1) << 3;
      break;
    case 7:
      _DAT_50440028 = _DAT_50440028 & 0xffffffef | ((uI)a2 & 1) << 4;
      break;
    case 8:
      _DAT_50440028 = _DAT_50440028 & 0xffffffdf | ((uI)a2 & 1) << 5;
      break;
    case 9:
      _DAT_5044002c = _DAT_5044002c & 0xfffffffe | (uI)a2 & 1;
      break;
    case 10:
      _DAT_5044002c = _DAT_5044002c & 0xfffffffd | ((uI)a2 & 1) << 1;
      break;
    case 0xb:
      _DAT_5044002c = _DAT_5044002c & 0xfffffffb | ((uI)a2 & 1) << 2;
      break;
    case 0xc:
      _DAT_5044002c = _DAT_5044002c & 0xfffffff7 | ((uI)a2 & 1) << 3;
      break;
    case 0xd:
      _DAT_5044002c = _DAT_5044002c & 0xffffffef | ((uI)a2 & 1) << 4;
      break;
    case 0xe:
      _DAT_5044002c = _DAT_5044002c & 0xffffffdf | ((uI)a2 & 1) << 5;
      break;
    case 0xf:
      _DAT_5044002c = _DAT_5044002c & 0xffffffbf | ((uI)a2 & 1) << 6;
      break;
    case 0x10:
      _DAT_5044002c = _DAT_5044002c & 0xffffff7f | ((uI)a2 & 1) << 7;
      break;
    case 0x11:
      _DAT_5044002c = _DAT_5044002c & 0xfffffeff | ((uI)a2 & 1) << 8;
      break;
    case 0x12:
      _DAT_5044002c = _DAT_5044002c & 0xfffffdff | ((uI)a2 & 1) << 9;
      break;
    case 0x13:
      _DAT_5044002c = _DAT_5044002c & 0xfffffbff | ((uI)a2 & 1) << 10;
      break;
    case 0x14:
      _DAT_5044002c = _DAT_5044002c & 0xfffff7ff | ((uI)a2 & 1) << 0xb;
      break;
    case 0x15:
      _DAT_5044002c = _DAT_5044002c & 0xffffefff | ((uI)a2 & 1) << 0xc;
      break;
    case 0x16:
      _DAT_5044002c = _DAT_5044002c & 0xffffdfff | ((uI)a2 & 1) << 0xd;
      break;
    case 0x17:
      _DAT_5044002c = _DAT_5044002c & 0xffffbfff | ((uI)a2 & 1) << 0xe;
      break;
    case 0x18:
      _DAT_5044002c = _DAT_5044002c & 0xffff7fff | ((uI)a2 & 1) << 0xf;
      break;
    case 0x19:
      _DAT_5044002c = _DAT_5044002c & 0xfffeffff | ((uI)a2 & 1) << 0x10;
      break;
    case 0x1a:
      _DAT_5044002c = _DAT_5044002c & 0xfffdffff | ((uI)a2 & 1) << 0x11;
      break;
    case 0x1b:
      _DAT_5044002c = _DAT_5044002c & 0xfffbffff | ((uI)a2 & 1) << 0x12;
      break;
    case 0x1c:
      _DAT_5044002c = _DAT_5044002c & 0xfff7ffff | ((uI)a2 & 1) << 0x13;
      break;
    case 0x1d:
      _DAT_5044002c = _DAT_5044002c & 0xffefffff | ((uI)a2 & 1) << 0x14;
      break;
    case 0x1e:
      _DAT_5044002c = _DAT_5044002c & 0xffdfffff | ((uI)a2 & 1) << 0x15;
      break;
    case 0x1f:
      _DAT_5044002c = _DAT_5044002c & 0xffbfffff | ((uI)a2 & 1) << 0x16;
      break;
    case 0x20:
      _DAT_5044002c = _DAT_5044002c & 0xff7fffff | ((uI)a2 & 1) << 0x17;
      break;
    case 0x21:
      _DAT_5044002c = _DAT_5044002c & 0xfeffffff | ((uI)a2 & 1) << 0x18;
      break;
    case 0x22:
      _DAT_5044002c = _DAT_5044002c & 0xfdffffff | ((uI)a2 & 1) << 0x19;
      break;
    case 0x23:
      _DAT_5044002c = _DAT_5044002c & 0xfbffffff | ((uI)a2 & 1) << 0x1a;
      break;
    case 0x24:
      _DAT_5044002c = _DAT_5044002c & 0xf7ffffff | ((uI)a2 & 1) << 0x1b;
      break;
    case 0x25:
      _DAT_5044002c = _DAT_5044002c & 0xdfffffff | ((uI)a2 & 1) << 0x1d;
      break;
    case 0x26:
      _DAT_5044002c = _DAT_5044002c & 0xbfffffff | ((uI)a2 & 1) << 0x1e;
      break;
    case 0x27:
      _DAT_5044002c = _DAT_5044002c & 0x7fffffff | (uI)a2 << 0x1f;
  }
  R 0;
}

undef8 sysctl_clock_enable(uI a1)
{
  undef8 uVar1;

  if (a1 < 0x2a) {
    sysctl_clock_bus_en((J)(I)a1, 1);
    sysctl_clock_device_en((J)(I)a1, 1);
    uVar1 = 0;
  } else {
    uVar1 = 0xffffffffffffffff;
  }
  R uVar1;
}


undef8 sysctl_clock_set_threshold(undef4 a1, uI a2)
{
  switch (a1) {
    case 1:
      _DAT_50440020 = _DAT_50440020 & 0xffffffc7 | (a2 & 7) << 3;
      break;
    case 2:
      _DAT_50440020 = _DAT_50440020 & 0xfffffe3f | (a2 & 7) << 6;
      break;
    case 3:
      _DAT_50440020 = _DAT_50440020 & 0xfffff1ff | (a2 & 7) << 9;
      break;
    case 4:
      _DAT_50440038 = _DAT_50440038 & 0xfffffff0 | a2 & 0xf;
      break;
    case 5:
      _DAT_50440038 = _DAT_50440038 & 0xffffff0f | (a2 & 0xf) << 4;
      break;
    case 6:
      _DAT_50440038 = _DAT_50440038 & 0xfffff0ff | (a2 & 0xf) << 8;
      break;
    case 7:
      _DAT_50440038 = _DAT_50440038 & 0xffff0fff | (a2 & 0xf) << 0xc;
      break;
    case 8:
      _DAT_50440038 = _DAT_50440038 & 0xfff0ffff | (a2 & 0xf) << 0x10;
      break;
    case 9:
      _DAT_5044003c = _DAT_5044003c & 0xffffff00 | a2 & 0xff;
      break;
    case 10:
      _DAT_5044003c = _DAT_5044003c & 0xffff00ff | (a2 & 0xff) << 8;
      break;
    case 0xb:
      _DAT_5044003c = _DAT_5044003c & 0xff00ffff | (a2 & 0xff) << 0x10;
      break;
    case 0xc:
      _DAT_5044003c = _DAT_5044003c & 0xffffff | a2 << 0x18;
      break;
    case 0xd:
      _DAT_50440040 = _DAT_50440040 & 0xffffff00 | a2 & 0xff;
      break;
    case 0xe:
      _DAT_50440040 = _DAT_50440040 & 0xffff00ff | (a2 & 0xff) << 8;
      break;
    case 0xf:
      _DAT_50440040 = _DAT_50440040 & 0xff00ffff | (a2 & 0xff) << 0x10;
      break;
    case 0x10:
      _DAT_50440044 = _DAT_50440044 & 0xffff0000 | a2 & 0xffff;
      break;
    case 0x11:
      _DAT_50440044 =
          _DAT_50440044 & 0xffff | (uI)((U)((J)(I)a2 << 0x30) >> 0x20);
      break;
    case 0x12:
      _DAT_50440048 = _DAT_50440048 & 0xffff0000 | a2 & 0xffff;
      break;
    case 0x13:
      _DAT_50440048 = _DAT_50440048 & 0xff00ffff | (a2 & 0xff) << 0x10;
      break;
    case 0x14:
      _DAT_50440048 = _DAT_50440048 & 0xffffff | a2 << 0x18;
      break;
    case 0x15:
      _DAT_5044004c = _DAT_5044004c & 0xffffff00 | a2 & 0xff;
      break;
    case 0x16:
      _DAT_5044004c = _DAT_5044004c & 0xffff00ff | (a2 & 0xff) << 8;
      break;
    case 0x17:
      _DAT_5044004c = _DAT_5044004c & 0xff00ffff | (a2 & 0xff) << 0x10;
      break;
    case 0x18:
      _DAT_5044004c = _DAT_5044004c & 0xffffff | a2 << 0x18;
      break;
    case 0x19:
      _DAT_50440050 = _DAT_50440050 & 0xffffff00 | a2 & 0xff;
      break;
    case 0x1a:
      _DAT_50440050 = _DAT_50440050 & 0xffff00ff | (a2 & 0xff) << 8;
      break;
    case 0x1b:
      _DAT_50440050 = _DAT_50440050 & 0xff00ffff | (a2 & 0xff) << 0x10;
  }
  R 0;
}


J sysctl_clock_get_threshold(undef4 a1)
{
  uI l14;

  l14 = 0;
  switch (a1) {
    case 0:
      l14 = _DAT_50440020 >> 1 & 3;
      break;
    case 1:
      l14 = _DAT_50440020 >> 3 & 7;
      break;
    case 2:
      l14 = _DAT_50440020 >> 6 & 7;
      break;
    case 3:
      l14 = _DAT_50440020 >> 9 & 7;
      break;
    case 4:
      l14 = _DAT_50440038 & 0xf;
      break;
    case 5:
      l14 = _DAT_50440038 >> 4 & 0xf;
      break;
    case 6:
      l14 = _DAT_50440038 >> 8 & 0xf;
      break;
    case 7:
      l14 = _DAT_50440038 >> 0xc & 0xf;
      break;
    case 8:
      l14 = _DAT_50440038 >> 0x10 & 0xf;
      break;
    case 9:
      l14 = _DAT_5044003c & 0xff;
      break;
    case 10:
      l14 = _DAT_5044003c >> 8 & 0xff;
      break;
    case 0xb:
      l14 = _DAT_5044003c >> 0x10 & 0xff;
      break;
    case 0xc:
      l14 = _DAT_5044003c >> 0x18;
      break;
    case 0xd:
      l14 = _DAT_50440040 & 0xff;
      break;
    case 0xe:
      l14 = _DAT_50440040 >> 8 & 0xff;
      break;
    case 0xf:
      l14 = _DAT_50440040 >> 0x10 & 0xff;
      break;
    case 0x10:
      l14 = (uI)((U)((J)(I)_DAT_50440044 << 0x30) >> 0x30);
      break;
    case 0x11:
      l14 = (uI)((U)((J)(I)(_DAT_50440044 >> 0x10) << 0x30) >> 0x30);
      break;
    case 0x12:
      l14 = (uI)((U)((J)(I)_DAT_50440048 << 0x30) >> 0x30);
      break;
    case 0x13:
      l14 = _DAT_50440048 >> 0x10 & 0xff;
      break;
    case 0x14:
      l14 = _DAT_50440048 >> 0x18;
      break;
    case 0x15:
      l14 = _DAT_5044004c & 0xff;
      break;
    case 0x16:
      l14 = _DAT_5044004c >> 8 & 0xff;
      break;
    case 0x17:
      l14 = _DAT_5044004c >> 0x10 & 0xff;
      break;
    case 0x18:
      l14 = _DAT_5044004c >> 0x18;
      break;
    case 0x19:
      l14 = _DAT_50440050 & 0xff;
      break;
    case 0x1a:
      l14 = _DAT_50440050 >> 8 & 0xff;
      break;
    case 0x1b:
      l14 = _DAT_50440050 >> 0x10 & 0xff;
  }
  R(J)(I) l14;
}


undef8 sysctl_clock_set_clock_select(undef4 a1, uI a2)
{
  switch (a1) {
    case 0:
      _DAT_50440008 = _DAT_50440008 & 0xff7fffff | (a2 & 1) << 0x17;
      break;
    case 1:
      _DAT_5044000c = _DAT_5044000c & 0xff7fffff | (a2 & 1) << 0x17;
      break;
    case 2:
      _DAT_50440010 = _DAT_50440010 & 0xff7fffff | (a2 & 1) << 0x17;
      break;
    case 3:
      _DAT_50440010 = _DAT_50440010 & 0xf3ffffff | (a2 & 3) << 0x1a;
      break;
    case 4:
      _DAT_50440020 = _DAT_50440020 & 0xfffffffe | a2 & 1;
      break;
    case 5:
      _DAT_50440020 = _DAT_50440020 & 0xffffefff | (a2 & 1) << 0xc;
      break;
    case 6:
      _DAT_50440020 = _DAT_50440020 & 0xffffdfff | (a2 & 1) << 0xd;
      break;
    case 7:
      _DAT_50440020 = _DAT_50440020 & 0xffffbfff | (a2 & 1) << 0xe;
      break;
    case 8:
      _DAT_50440020 = _DAT_50440020 & 0xffff7fff | (a2 & 1) << 0xf;
      break;
    case 9:
      _DAT_50440024 = _DAT_50440024 & 0xfffffffe | a2 & 1;
      break;
    case 10:
      _DAT_50440024 = _DAT_50440024 & 0x7fffffff | a2 << 0x1f;
  }
  R 0;
}


J sysctl_clock_get_clock_select(undef4 a1)
{
  uI l14;

  l14 = 0;
  switch (a1) {
    case 0:
      l14 = _DAT_50440008 >> 0x17 & 1;
      break;
    case 1:
      l14 = _DAT_5044000c >> 0x17 & 1;
      break;
    case 2:
      l14 = _DAT_50440010 >> 0x17 & 1;
      break;
    case 3:
      l14 = _DAT_50440010 >> 0x1a & 3;
      break;
    case 4:
      l14 = _DAT_50440020 & 1;
      break;
    case 5:
      l14 = _DAT_50440020 >> 0xc & 1;
      break;
    case 6:
      l14 = _DAT_50440020 >> 0xd & 1;
      break;
    case 7:
      l14 = _DAT_50440020 >> 0xe & 1;
      break;
    case 8:
      l14 = _DAT_50440020 >> 0xf & 1;
      break;
    case 9:
      l14 = _DAT_50440024 & 1;
      break;
    case 10:
      l14 = _DAT_50440024 >> 0x1f;
  }
  R(J)(I) l14;
}


J sysctl_clock_source_get_freq(undef4 a1)
{
  I l14;

  switch (a1) {
    case 0:
      l14 = 26000000;
      break;
    case 1:
      l14 = sysctl_pll_get_freq(0);
      break;
    case 2:
      l14 = sysctl_pll_get_freq(1);
      break;
    case 3:
      l14 = sysctl_pll_get_freq(2);
      break;
    case 4:
      l14 = sysctl_clock_get_freq(0x28);
      break;
    default:
      l14 = 0;
  }
  R(J) l14;
}


undef8 sysctl_pll_is_lock(uI a1)
{
  undef8 uVar1;
  C l11;

  l11 = 0;
  if (a1 < 3) {
    if (a1 == 1) {
      l11 = (C)((uI)_DAT_50440018 >> 8) & 3;
    } else {
      if (a1 == 0) {
        l11 = (C)_DAT_50440018 & 3;
      } else {
        if (a1 == 2) {
          l11 = (C)((uI)_DAT_50440018 >> 0x10) & 3;
        }
      }
    }
    if (l11 == 3) {
      uVar1 = 1;
    } else {
      uVar1 = 0;
    }
  } else {
    uVar1 = 0;
  }
  R uVar1;
}


undef8 sysctl_pll_clear_slip(uI a1)
{
  J lVar1;
  undef8 uVar2;

  if (a1 < 3) {
    if (a1 == 1) {
      _DAT_50440018 = _DAT_50440018 | 0x400;
    } else {
      if (a1 == 0) {
        _DAT_50440018 = _DAT_50440018 | 4;
      } else {
        if (a1 == 2) {
          _DAT_50440018 = _DAT_50440018 | 0x40000;
        }
      }
    }
    lVar1 = sysctl_pll_is_lock((J)(I)a1);
    if (lVar1 == 0) {
      uVar2 = 0xffffffffffffffff;
    } else {
      uVar2 = 0;
    }
  } else {
    uVar2 = 0xffffffffffffffff;
  }
  R uVar2;
}


undef8 sysctl_pll_enable(uI a1)
{
  undef8 uVar1;

  if (a1 < 3) {
    if (a1 == 1) {
      _DAT_5044000c = _DAT_5044000c & 0xff6fffff | 0x200000;
    } else {
      if (a1 == 0) {
        _DAT_50440008 = _DAT_50440008 & 0xff6fffff | 0x200000;
      } else {
        if (a1 == 2) {
          _DAT_50440010 = _DAT_50440010 & 0xff6fffff | 0x200000;
        }
      }
    }
    uVar1 = 0;
  } else {
    uVar1 = 0xffffffffffffffff;
  }
  R uVar1;
}


J sysctl_pll_get_freq(uI a1)
{
  C bVar1;
  J lVar2;
  I l20;

  l20 = 0;
  if (a1 < 3) {
    if (a1 == 1) {
      sysctl_clock_source_get_freq(0);
      l20 = (_DAT_5044000c >> 10 & 0xf) + 1;
    } else {
      if (a1 == 0) {
        sysctl_clock_source_get_freq(0);
        l20 = (_DAT_50440008 >> 10 & 0xf) + 1;
      } else {
        if (a1 == 2) {
          bVar1 = (C)(_DAT_50440010 >> 0x1a) & 3;
          if (2 < bVar1) {
            R 0;
          }
          sysctl_clock_source_get_freq(
              (J)(I)(uI)(C)(&DAT_8800b900)[(I)(uI)bVar1]);
          l20 = (_DAT_50440010 >> 10 & 0xf) + 1;
        }
      }
    }
    lVar2 = (J)l20;
  } else {
    lVar2 = 0;
  }
  R lVar2;
}


J sysctl_clock_get_freq(undef4 a1)
{
  I iVar1;
  I iVar2;
  J lVar3;
  uI l28;
  uI l24;

  l24 = 0;
  l28 = 0;
  switch (a1) {
    case 0:
      l28 = sysctl_clock_source_get_freq(1);
      break;
    case 1:
      l28 = sysctl_clock_source_get_freq(2);
      break;
    case 2:
      l28 = sysctl_clock_source_get_freq(3);
      break;
    case 3:
      lVar3 = sysctl_clock_get_clock_select(4);
      if (lVar3 == 0) {
        l24 = sysctl_clock_source_get_freq(0);
      } else {
        if (lVar3 == 1) {
          iVar1 = sysctl_clock_source_get_freq(1);
          iVar2 = sysctl_clock_get_threshold(0);
          l24 = (uI)(((J)iVar1 & 0xffffffffU) >> ((J)(iVar2 + 1) & 0x3fU));
        }
      }
      l28 = l24;
      break;
    case 4:
      l28 = sysctl_clock_source_get_freq(4);
      iVar1 = sysctl_clock_get_threshold(4);
      l28 = l28 / (iVar1 + 1U);
      break;
    case 5:
      l28 = sysctl_clock_source_get_freq(4);
      iVar1 = sysctl_clock_get_threshold(5);
      l28 = l28 / (iVar1 + 1U);
      break;
    case 6:
      l28 = sysctl_clock_source_get_freq(4);
      iVar1 = sysctl_clock_get_threshold(1);
      l28 = l28 / (iVar1 + 1U);
      break;
    case 7:
      l28 = sysctl_clock_source_get_freq(4);
      iVar1 = sysctl_clock_get_threshold(2);
      l28 = l28 / (iVar1 + 1U);
      break;
    case 8:
      l28 = sysctl_clock_source_get_freq(4);
      iVar1 = sysctl_clock_get_threshold(3);
      l28 = l28 / (iVar1 + 1U);
      break;
    case 9:
      l28 = sysctl_clock_source_get_freq(4);
      iVar1 = sysctl_clock_get_threshold(8);
      l28 = l28 / (iVar1 + 1U);
      break;
    case 10:
      lVar3 = sysctl_clock_get_clock_select(4);
      if (lVar3 == 0) {
        l24 = sysctl_clock_source_get_freq(0);
      } else {
        if (lVar3 == 1) {
          iVar1 = sysctl_clock_source_get_freq(1);
          iVar2 = sysctl_clock_get_threshold(0);
          l24 = (uI)(((J)iVar1 & 0xffffffffU) >> ((J)(iVar2 + 1) & 0x3fU));
        }
      }
      l28 = l24;
      break;
    case 0xb:
      l28 = sysctl_clock_source_get_freq(2);
      iVar1 = sysctl_clock_get_threshold(6);
      l28 = l28 / (iVar1 + 1U);
      break;
    case 0xc:
      l28 = sysctl_clock_source_get_freq(4);
      iVar1 = sysctl_clock_get_threshold(7);
      l28 = l28 / (iVar1 + 1U);
      break;
    case 0xd:
      lVar3 = sysctl_clock_get_clock_select(4);
      if (lVar3 == 0) {
        l24 = sysctl_clock_source_get_freq(0);
      } else {
        if (lVar3 == 1) {
          iVar1 = sysctl_clock_source_get_freq(1);
          iVar2 = sysctl_clock_get_threshold(0);
          l24 = (uI)(((J)iVar1 & 0xffffffffU) >> ((J)(iVar2 + 1) & 0x3fU));
        }
      }
      l28 = l24;
      break;
    case 0xe:
      l28 = sysctl_clock_get_freq(6);
      break;
    case 0xf:
      l28 = sysctl_clock_source_get_freq(1);
      iVar1 = sysctl_clock_get_threshold(9);
      l28 = l28 / (uI)((iVar1 + 1) * 2);
      break;
    case 0x10:
      l28 = sysctl_clock_source_get_freq(1);
      iVar1 = sysctl_clock_get_threshold(10);
      l28 = l28 / (uI)((iVar1 + 1) * 2);
      break;
    case 0x11:
      l28 = sysctl_clock_source_get_freq(1);
      iVar1 = sysctl_clock_get_threshold(0xb);
      l28 = l28 / (uI)((iVar1 + 1) * 2);
      break;
    case 0x12:
      lVar3 = sysctl_clock_get_clock_select(5);
      if (lVar3 == 0) {
        l24 = sysctl_clock_source_get_freq(0);
      } else {
        if (lVar3 == 1) {
          l24 = sysctl_clock_source_get_freq(1);
        }
      }
      iVar1 = sysctl_clock_get_threshold(0xc);
      l28 = l24 / (uI)((iVar1 + 1) * 2);
      break;
    case 0x13:
      l28 = sysctl_clock_source_get_freq(3);
      iVar1 = sysctl_clock_get_threshold(0x10);
      l28 = l28 / (uI)((iVar1 + 1) * 2);
      break;
    case 0x14:
      l28 = sysctl_clock_source_get_freq(3);
      iVar1 = sysctl_clock_get_threshold(0x11);
      l28 = l28 / (uI)((iVar1 + 1) * 2);
      break;
    case 0x15:
      l28 = sysctl_clock_source_get_freq(3);
      iVar1 = sysctl_clock_get_threshold(0x12);
      l28 = l28 / (uI)((iVar1 + 1) * 2);
      break;
    case 0x16:
      l28 = sysctl_clock_source_get_freq(1);
      iVar1 = sysctl_clock_get_threshold(0x16);
      l28 = l28 / (uI)((iVar1 + 1) * 2);
      break;
    case 0x17:
      l28 = sysctl_clock_source_get_freq(1);
      iVar1 = sysctl_clock_get_threshold(0x17);
      l28 = l28 / (uI)((iVar1 + 1) * 2);
      break;
    case 0x18:
      l28 = sysctl_clock_source_get_freq(1);
      iVar1 = sysctl_clock_get_threshold(0x18);
      l28 = l28 / (uI)((iVar1 + 1) * 2);
      break;
    case 0x19:
      l28 = sysctl_clock_get_freq(6);
      break;
    case 0x1a:
      l28 = sysctl_clock_get_freq(6);
      break;
    case 0x1b:
      l28 = sysctl_clock_get_freq(6);
      break;
    case 0x1c:
      l28 = sysctl_clock_get_freq(7);
      break;
    case 0x1d:
      l28 = sysctl_clock_get_freq(6);
      break;
    case 0x1e:
      lVar3 = sysctl_clock_get_clock_select(6);
      if (lVar3 == 0) {
        l24 = sysctl_clock_source_get_freq(0);
      } else {
        if (lVar3 == 1) {
          l24 = sysctl_clock_source_get_freq(1);
        }
      }
      iVar1 = sysctl_clock_get_threshold(0xd);
      l28 = l24 / (uI)((iVar1 + 1) * 2);
      break;
    case 0x1f:
      lVar3 = sysctl_clock_get_clock_select(7);
      if (lVar3 == 0) {
        l24 = sysctl_clock_source_get_freq(0);
      } else {
        if (lVar3 == 1) {
          l24 = sysctl_clock_source_get_freq(1);
        }
      }
      iVar1 = sysctl_clock_get_threshold(0xe);
      l28 = l24 / (uI)((iVar1 + 1) * 2);
      break;
    case 0x20:
      lVar3 = sysctl_clock_get_clock_select(8);
      if (lVar3 == 0) {
        l24 = sysctl_clock_source_get_freq(0);
      } else {
        if (lVar3 == 1) {
          l24 = sysctl_clock_source_get_freq(1);
        }
      }
      iVar1 = sysctl_clock_get_threshold(0xf);
      l28 = l24 / (uI)((iVar1 + 1) * 2);
      break;
    case 0x21:
      l28 = sysctl_clock_source_get_freq(0);
      iVar1 = sysctl_clock_get_threshold(0x19);
      l28 = l28 / (uI)((iVar1 + 1) * 2);
      break;
    case 0x22:
      l28 = sysctl_clock_source_get_freq(0);
      iVar1 = sysctl_clock_get_threshold(0x1a);
      l28 = l28 / (uI)((iVar1 + 1) * 2);
      break;
    case 0x23:
      l28 = sysctl_clock_get_freq(6);
      break;
    case 0x24:
      l28 = sysctl_clock_get_freq(7);
      break;
    case 0x25:
      l28 = sysctl_clock_get_freq(7);
      break;
    case 0x26:
      lVar3 = sysctl_clock_get_clock_select(4);
      if (lVar3 == 0) {
        l24 = sysctl_clock_source_get_freq(0);
      } else {
        if (lVar3 == 1) {
          iVar1 = sysctl_clock_source_get_freq(1);
          iVar2 = sysctl_clock_get_threshold(0);
          l24 = (uI)(((J)iVar1 & 0xffffffffU) >> ((J)(iVar2 + 1) & 0x3fU));
        }
      }
      l28 = l24;
      break;
    case 0x27:
      l28 = sysctl_clock_get_freq(7);
      break;
    case 0x28:
      lVar3 = sysctl_clock_get_clock_select(4);
      if (lVar3 == 0) {
        l24 = sysctl_clock_source_get_freq(0);
      } else {
        if (lVar3 == 1) {
          iVar1 = sysctl_clock_source_get_freq(1);
          iVar2 = sysctl_clock_get_threshold(0);
          l24 = (uI)(((J)iVar1 & 0xffffffffU) >> ((J)(iVar2 + 1) & 0x3fU));
        }
      }
      l28 = l24;
      break;
    case 0x29:
      lVar3 = sysctl_clock_get_clock_select(4);
      if (lVar3 == 0) {
        l24 = sysctl_clock_source_get_freq(0);
      } else {
        if (lVar3 == 1) {
          iVar1 = sysctl_clock_source_get_freq(1);
          iVar2 = sysctl_clock_get_threshold(0);
          l24 = (uI)(((J)iVar1 & 0xffffffffU) >> ((J)(iVar2 + 1) & 0x3fU));
        }
      }
      l28 = l24;
  }
  R(J)(I) l28;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same
// address

undef8 sysctl_pll_fast_enable_pll(void)
{
  J lVar1;

  _DAT_50440008 = _DAT_50440008 & 0xfff00000 | 0xec7b0;
  sysctl_pll_enable(0);
  while (lVar1 = sysctl_pll_is_lock(0), lVar1 == 0) {
    sysctl_pll_clear_slip(0);
  }
  sysctl_clock_enable(0);
  sysctl_clock_set_clock_select(4, 1);
  R 0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same
// address

undef8 uarths_putc(C a1)
{
  do {
  } while ((I)_DAT_38000000 < 0);
  _DAT_38000000 = _DAT_38000000 & 0xffffff00 | (uI)a1;
  R 0;
}

// WARNING: Globals starting with '_' overlap smaller symbols at the same
// address

J uarths_getc(void)
{
  do {
  } while ((_DAT_38000004 & 0x80000000) != 0);
  R(J)(I)(_DAT_38000004 & 0xff);
}

undef8 uarths_putchar(C a1)

{
  undef8 uVar1;

  uVar1 = uarths_putc((U)a1);
  R uVar1;
}

undef8 uarths_puts(C*a1)
{
  J lVar1;
  C*l18;

  l18 = a1;
  do {
    if (*l18 == 0) {
      R 0;
    }
    lVar1 = uarths_putc((U)*l18);
    l18 = l18 + 1;
  } while (lVar1 == 0);
  R 0xffffffffffffffff;
}


undef8 serial_init(void)
{
  uI uVar1;

  uVar1 = sysctl_get_freq();
  _DAT_38000008 = _DAT_38000008 & 0xfff8ffff | 1;
  _DAT_3800000c = _DAT_3800000c & 0xfff8ffff | 1;
  _DAT_38000010 = _DAT_38000010 & 0xfffffffe | 2;
  _DAT_38000014 = _DAT_38000014 | 3;
  _DAT_38000018 =
      _DAT_38000018 & 0xffff0000 | (uI)(ushort)((short)(uVar1 / 0x1c200) - 1);
  R 0;
}


void spi_receive_data_1(C*a1, C a2, undef *a3, uI a4)
{
  undef *l28;
  uI l20;
  C l19;
  C*l18;

  *(I*)(_DAT_805fc010 + 4) = a4 - 1;
  *(undef4 *)(_DAT_805fc010 + 8) = 1;
  l19 = a2;
  l18 = a1;
  while (l19 != '\0') {
    *(uI*)(_DAT_805fc010 + 0x60) = (uI)*l18;
    l19 = l19 + -1;
    l18 = l18 + 1;
  }
  *(undef4 *)(_DAT_805fc010 + 0x10) = 1;
  do {
    l28 = a3;
    l20 = a4;
  } while (*(uI*)(_DAT_805fc010 + 0x24) < a4);
  while (l20 != 0) {
    *l28 = (C) * (undef4 *)(_DAT_805fc010 + 0x60);
    l28 = l28 + 1;
    l20 = l20 - 1;
  }
  *(undef4 *)(_DAT_805fc010 + 0x10) = 0;
  *(undef4 *)(_DAT_805fc010 + 8) = 0;
  R;
}


void spi_send_data(C*a1, C a2, C*a3, I a4)
{
  C*l28;
  I l20;
  C l19;
  C*l18;

  *(undef4 *)(_DAT_805fc010 + 8) = 1;
  l19 = a2;
  l18 = a1;
  while (l28 = a3, l20 = a4, l19 != '\0') {
    *(uI*)(_DAT_805fc010 + 0x60) = (uI)*l18;
    l19 = l19 + -1;
    l18 = l18 + 1;
  }
  while (l20 != 0) {
    *(uI*)(_DAT_805fc010 + 0x60) = (uI)*l28;
    l28 = l28 + 1;
    l20 = l20 + -1;
  }
  *(undef4 *)(_DAT_805fc010 + 0x10) = 1;
  do {
  } while ((*(uI*)(_DAT_805fc010 + 0x28) & 5) != 4);
  *(undef4 *)(_DAT_805fc010 + 0x10) = 0;
  *(undef4 *)(_DAT_805fc010 + 8) = 0;
  R;
}


void spi_receive_data_2(undef4 *a1, C a2, undef *a3, uI a4)
{
  undef *l28;
  uI l20;
  C l19;
  undef4 *l18;

  *(I*)(_DAT_805fc010 + 4) = a4 - 1;
  *(undef4 *)(_DAT_805fc010 + 8) = 1;
  l19 = a2;
  l18 = a1;
  while (l19 != '\0') {
    *(undef4 *)(_DAT_805fc010 + 0x60) = *l18;
    l19 = l19 + -1;
    l18 = l18 + 1;
  }
  *(undef4 *)(_DAT_805fc010 + 0x10) = 1;
  do {
    l28 = a3;
    l20 = a4;
  } while (*(uI*)(_DAT_805fc010 + 0x24) < a4);
  while (l20 != 0) {
    *l28 = (C) * (undef4 *)(_DAT_805fc010 + 0x60);
    l28 = l28 + 1;
    l20 = l20 - 1;
  }
  *(undef4 *)(_DAT_805fc010 + 0x10) = 0;
  *(undef4 *)(_DAT_805fc010 + 8) = 0;
  R;
}


void w25qxx_write_enable(void)
{
  undef l18[8];

  l18[0] = 6;
  *_DAT_805fc010 = 7 << ((J)(I)(uI)DAT_805fc018 & 0x1fU) |
                   1 << ((J)(I)(uI)DAT_805fc019 & 0x1fU);
  spi_send_data(l18, 1, 0, 0);
  R;
}


void w25qxx_write_status_reg(undef a1, undef a2)
{
  undef l18;
  undef l17;
  undef l16;

  l18 = 1;
  l17 = a1;
  l16 = a2;
  w25qxx_write_enable();
  spi_send_data(&l18, 3, 0, 0);
  R;
}


void w25qxx_read_status_reg1(undef8 a1)
{
  undef l18[8];

  l18[0] = 5;
  *_DAT_805fc010 = 7 << ((J)(I)(uI)DAT_805fc018 & 0x1fU) |
                   3 << ((J)(I)(uI)DAT_805fc019 & 0x1fU);
  spi_receive_data_1(l18, 1, a1, 1);
  R;
}


void w25qxx_read_status_reg2(undef8 a1)
{
  undef l18[8];

  l18[0] = 0x35;
  *_DAT_805fc010 = 7 << ((J)(I)(uI)DAT_805fc018 & 0x1fU) |
                   3 << ((J)(I)(uI)DAT_805fc019 & 0x1fU);
  spi_receive_data_1(l18, 1, a1, 1);
  R;
}


void w25qxx_enable_quad_mode(void)
{
  C l12;
  C l11;

  w25qxx_read_status_reg2(&l12);
  if ((l12 & 2) == 0) {
    l12 = l12 | 2;
    w25qxx_read_status_reg1(&l11);
    w25qxx_write_status_reg((U)l11, (U)l12);
  }
  R;
}


void flash_read_mode0(I a1, J a2, uI a3)
{
  J l30;
  uI l28;
  I l24;
  undef l18;
  undef l17;
  undef l16;
  undef l15;

  *_DAT_805fc010 = 7 << ((J)(I)(uI)DAT_805fc018 & 0x1fU) |
                   3 << ((J)(I)(uI)DAT_805fc019 & 0x1fU);
  l18 = 3;
  l30 = a2;
  l28 = a3;
  l24 = a1;
  while (true) {
    if (l28 == 0) {
      R;
    }
    l17 = (undef)((uI)l24 >> 0x10);
    l16 = (undef)((uI)l24 >> 8);
    l15 = (undef)l24;
    if (l28 < 0x20) break;
    spi_receive_data_1(&l18, 4, l30, 0x20);
    l28 = l28 - 0x20;
    l24 = l24 + 0x20;
    l30 = l30 + 0x20;
  }
  spi_receive_data_1(&l18, 4, l30, (J)(I)l28);
  R;
}


void flash_read_mode_1_2(I a1, J a2, uI a3)
{
  J l30;
  uI l28;
  I l24;
  undef4 l18;
  I l14;

  l30 = a2;
  l28 = a3;
  l24 = a1;
  if (DAT_805fc01b == '\x02') {
    *_DAT_805fc010 = 7 << ((J)(I)(uI)DAT_805fc018 & 0x1fU) |
                     2 << ((J)(I)(uI)DAT_805fc019 & 0x1fU) |
                     2 << ((J)(I)(uI)DAT_805fc01a & 0x1fU);
    _DAT_805fc010[0x3d] = 0x2221;
    l18 = 0xeb;
  } else {
    *_DAT_805fc010 = 7 << ((J)(I)(uI)DAT_805fc018 & 0x1fU) |
                     2 << ((J)(I)(uI)DAT_805fc019 & 0x1fU) |
                     1 << ((J)(I)(uI)DAT_805fc01a & 0x1fU);
    _DAT_805fc010[0x3d] = 0x221;
    l18 = 0xbb;
  }
  while (true) {
    if (l28 == 0) {
      R;
    }
    l14 = l24 << 8;
    if (l28 < 0x20) break;
    spi_receive_data_2(&l18, 2, l30, 0x20);
    l28 = l28 - 0x20;
    l24 = l24 + 0x20;
    l30 = l30 + 0x20;
  }
  spi_receive_data_2(&l18, 2, l30, (J)(I)l28);
  R;
}


void flash_spi3_config(void)
{
  C l18;
  undef l17;

  l18 = -1;
  l17 = 0xff;
  _DAT_5044002c = _DAT_5044002c | 0x200;
  _DAT_5044003c = _DAT_5044003c & 0xffffff;
  _DAT_805fc010 = &DAT_54000000;
  _DAT_54000008 = 0;
  _DAT_54000010 = 0;
  _DAT_54000014 = 2;
  _DAT_5400002c = 0;
  DAT_805fc018 = 0;
  DAT_805fc019 = 10;
  DAT_805fc01a = 0x16;
  DAT_805fc01b = 0;
  _DAT_54000000 = 0x407;
  spi_send_data(&l18, 2, 0, 0);
  spi_flash_read_manufacturer_id(&l18);
  if (l18 != ' ') {
    DAT_805fc01b = 2;
    w25qxx_enable_quad_mode();
  }
  R;
}


void spi_flash_read_manufacturer_id(undef8 a1)
{
  undef l18[8];

  l18[0] = 0x9f;
  *_DAT_805fc010 = 7 << ((J)(I)(uI)DAT_805fc018 & 0x1fU) |
                   3 << ((J)(I)(uI)DAT_805fc019 & 0x1fU);
  spi_receive_data_1(l18, 1, a1, 1);
  R;
}

void flash_read(I a1, undef8 a2, I a3)
{
  if (DAT_805fc01b == '\0') {
    flash_read_mode0((J)a1, a2, (J)a3);
  } else {
    flash_read_mode_1_2((J)a1, a2, (J)a3);
  }
  R;
}


void flash_spi0_config(void)
{
  C l18;
  undef l17;

  l18 = -1;
  l17 = 0xff;
  _DAT_805fc010 = &DAT_52000000;
  _DAT_52000008 = 0;
  _DAT_52000010 = 0;
  _DAT_52000014 = 2;
  _DAT_5200002c = 0;
  DAT_805fc018 = 0x10;
  DAT_805fc019 = 8;
  DAT_805fc01a = 0x15;
  DAT_805fc01b = 0;
  _DAT_52000000 = 0x70100;
  spi_send_data(&l18, 2, 0, 0);
  spi_flash_read_manufacturer_id(&l18);
  if (l18 != ' ') {
    DAT_805fc01b = 2;
    w25qxx_enable_quad_mode();
  }
  R;
}

void spi_flash_set_read_mode(undef a1)
{
  DAT_805fc01b = a1;
  R;
}

J fcn_88009bc8_slip(C*a1, I a2)
{
  I l2c;
  C*l28;
  uI l18;
  I l14;

  l18 = 0xffffffff;
  l2c = a2;
  l28 = a1;
  while (l2c != 0) {
    l18 = l18 ^ *l28;
    l14 = 7;
    while (-1 < l14) {
      l18 = l18 >> 1 ^ -(l18 & 1) & 0xedb88320;
      l14 = l14 + -1;
    }
    l28 = l28 + 1;
    l2c = l2c + -1;
  }
  R(J)(I) ~l18;
}

void uarths_set_baudrate(uI a1)
{
  uI uVar1;

  uVar1 = sysctl_clock_get_freq(3);
  *(uI*)(DAT_8800b920 + 0x18) = *(uI*)(DAT_8800b920 + 0x18) & 0xffff0000 |
                                 (uI)(ushort)((short)(uVar1 / a1) - 1);
  R;
}

void slip_handle_pkt(I*a1, ushort *a2, J a3)
{
  I iVar1;
  ushort uVar2;
  I iVar3;
  ushort *l30;
  undef *l28;

  uVar2 = *a2;
  if (uVar2 == 0xc3) {
    if (*a1 != 0) {
      *a1 = 0;
      *(undef *)(a3 + 1) = 0xe3;
      R;
    }
    iVar1 = *(I*)(a2 + 2);
    iVar3 = fcn_88009bc8_slip(a2 + 4, (J)(*(I*)(a2 + 6) + 8));
    if (iVar1 != iVar3) {
      *(undef *)(a3 + 1) = 0xe2;
      R;
    }
    l28 = (undef *)((J) * (I*)(a2 + 4) & 0xffffffff);
    l30 = a2 + 8;
    while (true) {
      iVar3 = *(I*)(a2 + 6);
      iVar1 = iVar3 + -1;
      *(C*)(a2 + 6) = (C)iVar1;
      *(undef *)((J)a2 + 0xd) = (C)((uI)iVar1 >> 8);
      *(C*)(a2 + 7) = (C)((uI)iVar1 >> 0x10);
      *(undef *)((J)a2 + 0xf) = (C)((uI)iVar1 >> 0x18);
      if (iVar3 == 0) break;
      *l28 = *(undef *)l30;
      l30 = (ushort *)((J)l30 + 1);
      l28 = l28 + 1;
    }
    *(undef *)(a3 + 1) = 0xe0;
    *a1 = 0;
    R;
  }
  if (uVar2 < 0xc4) {
    if (uVar2 == 0xc2) {
      *(undef *)(a3 + 1) = 0xe0;
      R;
    }
  } else {
    if (uVar2 == 0xc5) {
      if (*a1 != 0) {
        *a1 = 0;
        *(undef *)(a3 + 1) = 0xe3;
        R;
      }
      (*(code *)((J) * (I*)(a2 + 4) & 0xfffffffe))();
      *(undef *)(a3 + 1) = 0xe0;
      R;
    }
    if (uVar2 == 0xc6) {
      if (*(I*)(a2 + 6) != 4) {
        *(undef *)(a3 + 1) = 0xe1;
        R;
      }
      uarths_set_baudrate((J) * (I*)(a2 + 8));
      *(undef *)(a3 + 1) = 0xe0;
      R;
    }
  }
  *(undef *)(a3 + 1) = 0xe3;
  R;
}

void handle_memory_write(void)
{
  I iVar1;
  J unaff_s0;
  I iVar2;
  J lVar3;

  if (**(I**)(unaff_s0 + -0x38) == 0) {
    iVar1 = *(I*)(*(J*)(unaff_s0 + -0x40) + 4);
    iVar2 = fcn_88009bc8_slip(*(J*)(unaff_s0 + -0x40) + 8,
                              (J)(*(I*)(*(J*)(unaff_s0 + -0x40) + 0xc) + 8));
    if (iVar1 == iVar2) {
      *(U *)(unaff_s0 + -0x28) =
          (J) * (I*)(*(J*)(unaff_s0 + -0x40) + 8) & 0xffffffff;
      *(J*)(unaff_s0 + -0x30) = *(J*)(unaff_s0 + -0x40) + 0x10;
      while (true) {
        iVar2 = *(I*)(*(J*)(unaff_s0 + -0x40) + 0xc);
        iVar1 = iVar2 + -1;
        lVar3 = *(J*)(unaff_s0 + -0x40);
        *(undef *)(lVar3 + 0xc) = (C)iVar1;
        *(undef *)(lVar3 + 0xd) = (C)((uI)iVar1 >> 8);
        *(undef *)(lVar3 + 0xe) = (C)((uI)iVar1 >> 0x10);
        *(undef *)(lVar3 + 0xf) = (C)((uI)iVar1 >> 0x18);
        if (iVar2 == 0) break;
        **(undef **)(unaff_s0 + -0x28) = **(undef **)(unaff_s0 + -0x30);
        *(J*)(unaff_s0 + -0x30) = *(J*)(unaff_s0 + -0x30) + 1;
        *(J*)(unaff_s0 + -0x28) = *(J*)(unaff_s0 + -0x28) + 1;
      }
      *(undef *)(*(J*)(unaff_s0 + -0x48) + 1) = 0xe0;
      **(undef4 **)(unaff_s0 + -0x38) = 0;
    } else {
      *(undef *)(*(J*)(unaff_s0 + -0x48) + 1) = 0xe2;
    }
  } else {
    **(undef4 **)(unaff_s0 + -0x38) = 0;
    *(undef *)(*(J*)(unaff_s0 + -0x48) + 1) = 0xe3;
  }
  R;
}

void handle_baud_rate(void)
{
  J unaff_s0;

  if (*(I*)(*(J*)(unaff_s0 + -0x40) + 0xc) == 4) {
    uarths_set_baudrate((J) * (I*)(*(J*)(unaff_s0 + -0x40) + 0x10));
    *(undef *)(*(J*)(unaff_s0 + -0x48) + 1) = 0xe0;
  } else {
    *(undef *)(*(J*)(unaff_s0 + -0x48) + 1) = 0xe1;
  }
  R;
}

void handle_memory_boot(void)
{
  J unaff_s0;

  if (**(I**)(unaff_s0 + -0x38) == 0) {
    (*(code *)((J) * (I*)(*(J*)(unaff_s0 + -0x40) + 8) & 0xfffffffe))();
    *(undef *)(*(J*)(unaff_s0 + -0x48) + 1) = 0xe0;
  } else {
    **(undef4 **)(unaff_s0 + -0x38) = 0;
    *(undef *)(*(J*)(unaff_s0 + -0x48) + 1) = 0xe3;
  }
  R;
}

void handle_nop(void)
{
  J unaff_s0;

  *(undef *)(*(J*)(unaff_s0 + -0x48) + 1) = 0xe0;
  R;
}

undef8 fcn_8800a160_slip(J a1, C a2)
{
  I iVar1;
  J lVar2;
  short l12;

  lVar2 = *(J*)(a1 + 0x1008);
  l12 = slip_unescape((U)a2, a1 + 0x101c);
  if (-1 < l12) {
    iVar1 = *(I*)(a1 + 0x1018);
    *(I*)(a1 + 0x1018) = iVar1 + 1;
    *(undef *)(lVar2 + ((J)iVar1 & 0xffffffffU)) = (C)l12;
    if (0x410 < *(uI*)(a1 + 0x1018)) {
      l12 = 0;
      *(undef4 *)(a1 + 0x1020) = 0xe1;
    }
  }
  if (l12 == -2) {
    if (*(J*)(a1 + 0x1008) == a1 + 4) {
      *(J*)(a1 + 0x1010) = a1 + 4;
      *(J*)(a1 + 0x1008) = a1 + 0x804;
    } else {
      *(J*)(a1 + 0x1010) = a1 + 0x804;
      *(J*)(a1 + 0x1008) = a1 + 4;
    }
  }
  R 0;
}

void isp_run(void)
{
  I iVar1;
  undef uStack4176;
  undef uStack4175;
  undef uStack4168;
  undef uStack4167;
  undef auStack4160[4];
  undef auStack4156[4100];
  undef *puStack56;
  undef2 *puStack48;
  undef4 uStack40;
  I iStack32;
  C l12;
  C l11;

  l11 = otp_check_fuse_a_bit(2);
  if ((l11 == '\x10') && (iVar1 = otp_check_fuse_b_bit(7), iVar1 == 0xe)) {
    exit(7);
  }
  memset(auStack4160, 0, 0x1028);
  memset(auStack4160, 0, 0x1028);
  puStack56 = auStack4156;
  l12 = 0;
LAB_8800a3dc:
  l12 = uarths_getc();
  fcn_8800a160_slip(auStack4160, (U)l12);
  if (puStack48 == (undef2 *)0x0) goto code_r0x8800a424;
  goto LAB_8800a440;
code_r0x8800a424:
  if (iStack32 != 0) {
  LAB_8800a440:
    if (puStack48 != (undef2 *)0x0) {
      uStack4168 = (undef)*puStack48;
      uStack4167 = 0xe0;
      slip_handle_pkt(auStack4160, puStack48, &uStack4168);
      puStack48 = (undef2 *)0x0;
      uStack40 = 0;
      slip_sendpkt(&uStack4168, 2);
    }
    if (iStack32 != 0) {
      uStack4176 = 0xc1;
      uStack4175 = (undef)iStack32;
      iStack32 = 0;
      uStack40 = 0;
      slip_sendpkt(&uStack4176, 2);
    }
  }
  goto LAB_8800a3dc;
}

// WARNING: Removing unreachable block (ram,0x8800a60c)

void boot_main(void)
{
  C bVar1;
  I iVar2;
  undef8 *puVar3;
  J in_mhartid;
  undef8 l128;
  undef8 l120;
  undef8 l118;
  undef8 l110;
  undef8 l108;
  undef8 l100;
  ushort local_f6;
  I local_f4;
  undef auStack240[116];
  uI l7c;
  C acStack120[32];
  C acStack88[37];
  C l33;
  C l32;
  C l31;
  J l30;
  undef8 *l20;
  I l18;
  I l14;

  l30 = in_mhartid;
  l31 = otp_check_fuse_a_bit(2);
  if (l31 == '\x10') {
    otp_reset(0);
    local_f6 = 0;
    otp_read(0, &local_f4, 4);
    if (local_f4 == 0x4d584f52) {
      otp_read(4, &local_f6, 2);
      otp_read(6, &SUB_805f8000, (J)(I)(uI)local_f6);
      (*(code *)&SUB_805f8000)();
    }
    iVar2 = otp_check_fuse_b_bit(0x3f);
    if (iVar2 == 0xe) {
      turbo_mode_boot();
    }
  }
  if (((l31 != '\x10') || (iVar2 = otp_check_fuse_b_bit(7), iVar2 != 0xe)) &&
      (DAT_8800b8c0[1] = DAT_8800b8c0[1] | 1, (*DAT_8800b8c0 & 1) == 0)) {
    isp_run();
  }
  l33 = 0;
  flash_spi3_config();
  flash_read(0, &l33, 1);
  flash_read(1, &l7c, 4);
  if (0x5fc000 < l7c) {
    exit(0xe9);
  }
  flash_read(5, 0x80000000, (J)(I)l7c);
  flash_read((J)(I)(l7c + 5), acStack120, 0x20);
  if ((l31 == '\x10') && (iVar2 = otp_check_fuse_b_bit(9), iVar2 == 0xe)) {
    l32 = '\x01';
  } else {
    l32 = '\0';
  }
  if (l32 != '\0') {
    otp_read(0x3db0, acStack88, 0x20);
    l14 = 0;
    while (l14 < 0x20) {
      if (acStack120[l14] != acStack88[l14]) {
        isp_run();
      }
      l14 = l14 + 1;
    }
  }
  sha256_init(0, 0, (J)(I)(l7c + 5), auStack240);
  sha256_update(auStack240, &l33, 1);
  sha256_update(auStack240, &l7c, 4);
  sha256_update(auStack240, 0x80000000, (J)(I)l7c);
  sha256_final(auStack240, acStack88);
  l18 = 0;
  while (l18 < 0x20) {
    if (acStack120[l18] != acStack88[l18]) {
      isp_run();
    }
    l18 = l18 + 1;
  }
  if (((l31 == '\x10') && (iVar2 = otp_check_fuse_b_bit(8), iVar2 != 0xe)) &&
      ((l33 & 1) != 0)) {
    otp_set_0c();
    l108 = 0;
    l100 = 0;
    l118 = 0;
    l110 = 0;
    l20 = (undef8 *)&SUB_80000000;
    sysctl_clock_enable(0x1c);
    sysctl_reset(0x14);
    aes_init(&l108, 0x10, &l118, 0x10, 0, 1, 1, 0);
    while (l7c != 0) {
      aes_process_outer(l20, &l128, 0x10, 1);
      puVar3 = l20 + 1;
      *l20 = l128;
      l20 = l20 + 2;
      *puVar3 = l120;
      l7c = l7c - 0x10;
    }
    otp_clear_0c();
  }
  if (l30 == 0) {
    clI_ipi_send(1);
  }
  bVar1 = (*(code *)&SUB_80000000)();
  if (bVar1 == 0xc0) {
    uarths_putchar(0xdb);
    uarths_putchar(0xdc);
  } else {
    if (bVar1 == 0xdb) {
      uarths_putchar(0xdb);
      uarths_putchar(0xdd);
    } else {
      uarths_putchar((U)bVar1);
    }
  }
  R;
}

void slip_sendch(C a1)

{
  if (a1 == 0xc0) {
    uarths_putchar(0xdb);
    uarths_putchar(0xdc);
  } else {
    if (a1 == 0xdb) {
      uarths_putchar(0xdb);
      uarths_putchar(0xdd);
    } else {
      uarths_putchar((U)a1);
    }
  }
  R;
}

void slip_start(void)
{
  uarths_putchar(0xc0);
  R;
}

void slip_sendinner(J a1, U a2)
{
  I l14;

  l14 = 0;
  while ((U)(J)l14 < a2) {
    slip_sendch((U) * (C*)(a1 + l14));
    l14 = l14 + 1;
  }
  R;
}

void slip_sendpkt(undef8 a1, U a2)
{
  slip_start();
  slip_sendinner(a1, a2 & 0xffffffff);
  slip_start();
  R;
}

J slip_unescape(C a1, I*a2)
{
  I iVar1;
  J lVar2;

  if (a1 == 0xc0) {
    if (*a2 == 0) {
      *a2 = 1;
      lVar2 = -1;
    } else {
      *a2 = 0;
      lVar2 = -2;
    }
  } else {
    iVar1 = *a2;
    if (iVar1 == 1) {
      if (a1 == 0xdb) {
        *a2 = 2;
        lVar2 = -1;
      } else {
        lVar2 = (J)(I)(short)(ushort)a1;
      }
    } else {
      if (iVar1 == 0) {
        lVar2 = -1;
      } else {
        if (iVar1 == 2) {
          if (a1 == 0xdc) {
            *a2 = 1;
            lVar2 = 0xc0;
          } else {
            if (a1 == 0xdd) {
              *a2 = 1;
              lVar2 = 0xdb;
            } else {
              lVar2 = -1;
            }
          }
        } else {
          lVar2 = -1;
        }
      }
    }
  }
  R lVar2;
}

void FUN_8800acdc(uI a1)
{
  uI uVar1;

  uVar1 = sysctl_clock_get_freq(3);
  *(uI*)(DAT_8800b920 + 0x18) = *(uI*)(DAT_8800b920 + 0x18) & 0xffff0000 |
                                 (uI)(ushort)((short)(uVar1 / a1) - 1);
  R;
}

undef8 turbo_mode_boot(void)
{
  undef8 uVar1;
  C l11;

  l11 = 0;
  sysctl_clock_set_clock_select(5, 1);
  sysctl_clock_set_threshold(0xc, 3);
  spi_flash_set_read_mode(0);
  spi_flash_read_manufacturer_id(&l11);
  if (l11 == 0xff) {
    uVar1 = 1;
  } else {
    flash_read(0, &l11, 1);
    if ((l11 >> 1 & 1) == 0) {
      spi_flash_set_read_mode(2);
    } else {
      spi_flash_set_read_mode(1);
    }
    uVar1 = 0;
  }
  R uVar1;
}

undef8 flash_spi0_init(void)
{
  undef8 uVar1;
  C l11;

  l11 = 0;
  fpioa_init();
  sysctl_clock_enable(0x1d);
  sysctl_reset(7);
  sysctl_clock_enable(0xf);
  sysctl_clock_set_threshold(9, 7);
  fpioa_set_function(0x18, 0xc);
  fpioa_set_function(0x1b, 0x11);
  fpioa_set_function(0x19, 4);
  fpioa_set_function(0x1a, 5);
  fpioa_set_function(0x1c, 6);
  fpioa_set_function(0x1d, 7);
  flash_spi0_config();
  spi_flash_set_read_mode(0);
  spi_flash_read_manufacturer_id(&l11);
  if (l11 == 0xff) {
    uVar1 = 1;
  } else {
    flash_read(0, &l11, 1);
    if ((l11 >> 1 & 1) == 0) {
      spi_flash_set_read_mode(2);
    } else {
      spi_flash_set_read_mode(1);
    }
    uVar1 = 0;
  }
  R uVar1;
}

void turbo_mode_boot(void)
{
  undef uVar1;
  undef8 uVar2;
  undef8 uVar3;
  I iVar4;
  J lVar5;
  undef8 *extraout_a1;
  undef8 *puVar6;
  undef8 *puVar7;
  undef8 *puVar8;
  undef8 *puVar9;
  undef8 uVar10;
  undef8 uVar11;
  undef8 uVar12;
  undef8 uVar13;
  undef8 uVar14;
  undef8 uVar15;
  J in_mhartid;
  undef8 l120;
  undef8 l118;
  undef8 l110;
  undef8 l108;
  undef8 l100;
  undef8 local_f8;
  undef auStack240[116];
  uI l7c;
  C acStack120[32];
  C acStack88[38];
  C l32;
  bool l31;
  J l30;
  undef8 *l20;
  I l18;
  I l14;

  l30 = in_mhartid;
  iVar4 = otp_check_fuse_a_bit(2);
  if (iVar4 != 0x10) {
    exit(0xeb);
  }
  sysctl_pll_fast_enable_pll();
  FUN_8800acdc(0x1c200);
  otp_reset(0xe);
  iVar4 = otp_check_fuse_b_bit(7);
  if ((iVar4 != 0xe) &&
      (DAT_8800b8c0[1] = DAT_8800b8c0[1] | 1, (*DAT_8800b8c0 & 1) == 0)) {
    isp_run();
  }
  l32 = 0;
  flash_spi3_config();
  iVar4 = otp_check_fuse_b_bit(0x3e);
  if (iVar4 == 0xe) {
    lVar5 = turbo_mode_boot();
    if (lVar5 != 0) {
      flash_spi0_init();
    }
  } else {
    iVar4 = otp_check_fuse_b_bit(0x3d);
    if (iVar4 == 0xe) {
      flash_spi0_init();
    } else {
      turbo_mode_boot();
    }
  }
  flash_read(0, &l32, 1);
  flash_read(1, &l7c, 4);
  if (0x5fc000 < l7c) {
    exit(0xe9);
  }
  flash_read(5, 0x80000000, (J)(I)l7c);
  flash_read((J)(I)(l7c + 5), acStack120, 0x20);
  iVar4 = otp_check_fuse_b_bit(9);
  l31 = iVar4 == 0xe;
  if (l31) {
    otp_read(0x3db0, acStack88, 0x20);
    l14 = 0;
    while (l14 < 0x20) {
      if (acStack120[l14] != acStack88[l14]) {
        isp_run();
      }
      l14 = l14 + 1;
    }
  }
  sha256_init(0, 0, (J)(I)(l7c + 5), auStack240);
  sha256_update(auStack240, &l32, 1);
  sha256_update(auStack240, &l7c, 4);
  puVar7 = (undef8 *)(J)(I)l7c;
  sha256_update(auStack240, 0x80000000);
  sha256_final(auStack240, acStack88);
  l18 = 0;
  while (l18 < 0x20) {
    if (acStack120[l18] != acStack88[l18]) {
      isp_run();
    }
    l18 = l18 + 1;
  }
  iVar4 = otp_check_fuse_b_bit(8);
  if ((iVar4 != 0xe) && ((l32 & 1) != 0)) {
    otp_set_0c();
    l100 = 0;
    local_f8 = 0;
    l110 = 0;
    l108 = 0;
    l20 = (undef8 *)&SUB_80000000;
    sysctl_clock_enable(0x1c);
    sysctl_reset(0x14);
    puVar7 = &l110;
    aes_init(&l100, 0x10, 0x10, 0, 1, 1, 0);
    while (l7c != 0) {
      puVar7 = (undef8 *)&DAT_00000010;
      aes_process_outer(l20, &l120, 1);
      puVar6 = l20 + 1;
      *l20 = l120;
      l20 = l20 + 2;
      *puVar6 = l118;
      l7c = l7c - 0x10;
    }
    otp_clear_0c();
  }
  if (l30 == 0) {
    clI_ipi_send(1);
  }
  puVar9 = (undef8 *)(*(code *)&SUB_80000000)();
  puVar8 = (undef8 *)((J)puVar9 + (J)puVar7);
  puVar6 = extraout_a1;
  if (((((U)extraout_a1 ^ (U)puVar9) & 7) == 0) &&
      ((undef8 *)0x7 < puVar7)) {
    if (((U)puVar9 & 7) != 0) {
      while (((U)puVar9 & 7) != 0) {
        uVar1 = *(undef *)puVar6;
        puVar6 = (undef8 *)((J)puVar6 + 1);
        *(undef *)puVar9 = uVar1;
        puVar9 = (undef8 *)((J)puVar9 + 1);
      }
    }
    while (puVar9 < (undef8 *)((U)puVar8 & 0xfffffffffffffff8) + -8) {
      uVar2 = puVar6[1];
      uVar15 = puVar6[2];
      uVar14 = puVar6[3];
      uVar13 = puVar6[4];
      uVar12 = puVar6[5];
      uVar3 = puVar6[6];
      uVar11 = puVar6[7];
      uVar10 = puVar6[8];
      *puVar9 = *puVar6;
      puVar9[1] = uVar2;
      puVar9[2] = uVar15;
      puVar9[3] = uVar14;
      puVar9[4] = uVar13;
      puVar9[5] = uVar12;
      puVar9[6] = uVar3;
      puVar9[7] = uVar11;
      puVar9[8] = uVar10;
      puVar6 = puVar6 + 9;
      puVar9 = puVar9 + 9;
    }
    while (puVar9 < (undef8 *)((U)puVar8 & 0xfffffffffffffff8)) {
      uVar2 = *puVar6;
      puVar6 = puVar6 + 1;
      *puVar9 = uVar2;
      puVar9 = puVar9 + 1;
    }
    if (puVar8 <= puVar9) {
      R;
    }
  } else {
    if (puVar8 <= puVar9) {
      R;
    }
  }
  while (puVar9 < puVar8) {
    uVar1 = *(undef *)puVar6;
    puVar6 = (undef8 *)((J)puVar6 + 1);
    *(undef *)puVar9 = uVar1;
    puVar9 = (undef8 *)((J)puVar9 + 1);
  }
  R;
}

void memcpy(undef8 *a1, undef8 *a2, U a3)
{
  undef uVar1;
  undef8 uVar2;
  undef8 uVar3;
  undef8 *puVar4;
  undef8 uVar5;
  undef8 uVar6;
  undef8 uVar7;
  undef8 uVar8;
  undef8 uVar9;
  undef8 uVar10;

  puVar4 = (undef8 *)((J)a1 + a3);
  if (((((U)a2 ^ (U)a1) & 7) == 0) && (7 < a3)) {
    if (((U)a1 & 7) != 0) {
      while (((U)a1 & 7) != 0) {
        uVar1 = *(undef *)a2;
        a2 = (undef8 *)((J)a2 + 1);
        *(undef *)a1 = uVar1;
        a1 = (undef8 *)((J)a1 + 1);
      }
    }
    while (a1 < (undef8 *)((U)puVar4 & 0xfffffffffffffff8) + -8) {
      uVar2 = a2[1];
      uVar10 = a2[2];
      uVar9 = a2[3];
      uVar8 = a2[4];
      uVar7 = a2[5];
      uVar3 = a2[6];
      uVar6 = a2[7];
      uVar5 = a2[8];
      *a1 = *a2;
      a1[1] = uVar2;
      a1[2] = uVar10;
      a1[3] = uVar9;
      a1[4] = uVar8;
      a1[5] = uVar7;
      a1[6] = uVar3;
      a1[7] = uVar6;
      a1[8] = uVar5;
      a2 = a2 + 9;
      a1 = a1 + 9;
    }
    while (a1 < (undef8 *)((U)puVar4 & 0xfffffffffffffff8)) {
      uVar2 = *a2;
      a2 = a2 + 1;
      *a1 = uVar2;
      a1 = a1 + 1;
    }
    if (puVar4 <= a1) {
      R;
    }
  } else {
    if (puVar4 <= a1) {
      R;
    }
  }
  while (a1 < puVar4) {
    uVar1 = *(undef *)a2;
    a2 = (undef8 *)((J)a2 + 1);
    *(undef *)a1 = uVar1;
    a1 = (undef8 *)((J)a1 + 1);
  }
  R;
}

void memset(U*a1, U a2, U a3)
{
  U uVar1;
  U *puVar2;
  U uVar3;

  uVar1 = 0xf;
  if (0xf < a3) {
    uVar3 = (U)a1 & 0xf;
    if (uVar3 != 0) {
      a2 = (*(code *)(uVar3 * 4 + 0x8800b41c))();
      a1 = (U *)((J)a1 - (uVar3 - 0x10));
      a3 = a3 + (uVar3 - 0x10);
      if (a3 <= uVar1) goto LAB_8800b410;
    }
    if (a2 != 0) {
      a2 = a2 & 0xff | (a2 & 0xff) << 8;
      a2 = a2 | a2 << 0x10;
      a2 = a2 | a2 << 0x20;
    }
    uVar3 = a3 & 0xfffffffffffffff0;
    a3 = a3 & 0xf;
    puVar2 = (U *)(uVar3 + (J)a1);
    do {
      *a1 = a2;
      a1[1] = a2;
      a1 = a1 + 2;
    } while (a1 < puVar2);
    if (a3 == 0) {
      R;
    }
  }
LAB_8800b410:
  // WARNING: Could not recover jumptable at 0x8800b41c. Too many branches
  // WARNING: Treating indirect jump as call
  (*(code *)((uVar1 - a3) * 4 + 0x8800b420))();
  R;
}

C*strlen(C*a1)
{
  C cVar1;
  C*pcVar2;

  pcVar2 = a1;
  do {
    pcVar2 = pcVar2 + 1;
    cVar1 = *pcVar2;
    pcVar2 = pcVar2;
  } while (cVar1 != '\0');
  R pcVar2 + (-1 - (J)a1);
}



//:~