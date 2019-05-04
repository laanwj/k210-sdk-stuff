Maix Go / K210 stuff
=====================

First, get the Kendryte C toolchain and copy or symlink the contents of the
`src/` folder to a checkout of `https://github.com/sipeed/LicheeDan_K210_examples.git`.

Then to build a certain project do:

```bash
mkdir build && cd build
cmake .. -DPROJ=<ProjectName> -DTOOLCHAIN=/opt/riscv-toolchain/bin && make
```

You will get 2 files, `build/<ProjectName>` and `build/<ProjectName>.bin`. The former
is an ELF executable, the latter a raw binary that can be flashed or written to
0x80000000 in SRAM and directly executed.

Documentation
==============

Additional register documentation that is not in the datasheet can be found here:

- [K210 memory map](doc/memory_map.md) - A rough memory map for the Kendryte K210 (as used on the Sipeed Maix boards)
- [k210.svd](https://github.com/riscv-rust/k210-pac/blob/master/k210.svd) - Peripheral description for rust K210 BSP (k210-pac project)

Projects
=========

glyph_mapping
-------------

Variation of the `DVP` sample that processes the camera input through a simple
DOS 8×8 font glyph-mapping algorithm and shows it on the display.

[README](src/glyph_mapping/README.md)

dump_otp
--------

Dumps the contents of the OTP (One-Time Programmable memory) of the K210 CPU to
serial output in Intel HEX format.

[README](src/dump_otp/README.md)
