OTP layout
==========

Some random notes about the layout of the Kendryte K210 OTP memory.

There is some on-board One-Time Programmable Memory, which is used
to configure the boot sequence as used by the boot ROM. A part of it is pre-written
in the factory.

The total size seems to be 16384 (0x4000) bytes.

```
Ofs   Type   Description
----------------------------------------------------
0000  u32    "ROXM" (0x4d584f52) magic value
0004  u16    Size of "boot patch" program
0006  u8[sz] Program data to be copied
3d91  u8[11] Looks like a production run identifier (ASCII)
3d9c  u32?   Serial number? Differs per board instance
3da0  u16    ?
3da2  u16    ?
3da4  u16    ?
3da6  u16    ?
3fd0  u8[16] 2 bits per entry, for 64 entries
3fe0  u8[16] 2 bits per entry, for 64 entries
3ff0  u8[16] Causes read errors. Might be write-only area for encryption key.
```

- The "boot patch" program is copied to SRAM at address 0x805f8000 at boot and
  executed. I think it's meant to be a vector to fix issues in the ROM without
  having to modify the chip.

- You can read the OTP of your board using the `otp_dump` tool in this
  repository.  Note that the OTP contains a serial number (at least
  0x3d9c..0x3d9f seem to differ between boards) so it'd be wise to treat the
  output as privacy-sensitive.

- My intuition is that the areas at 0x3fd0 and 0x3fe0 are bit fields that specify
  which parts of the OTP have been written and which have not.

- Be careful: any writing to the OTP (if possible at all, I don't know, there
  are some routines in the ROM that look like they might do this but haven't
  tried !) might brick your chip unrecoverably.
