  entry
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
                                                       entry_hart0: 88000128(*)
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
