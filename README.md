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
- [OTP memory map](doc/otp_layout.md) - Some random notes about the layout of the Kendryte K210 OTP memory
- [LicheeDan / MaixGo on-board peripherals](doc/onboard.md) - List of on-board peripherals and chips for the various boards with a K210

External:

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

rust/k210-console
-----------------

Console emulator written in Rust for the Maix Go.

Barely functional at the moment. This is really a test for some functionality
like SPI and driving the display from Rust, and for playing with Rust RISC-V 64
in general.

[README](rust/k210-console/README.md)

rust/mandelbrot
---------------

Mandelbrot fractal zoom.

[README](rust/mandelbrot/README.md)

ROM re'ing
===========

Annotated radare2 config files for the Boot ROM and OTP can be found under [r2](r2/README.md).

Other projects
==============

Some other interesting K210-based projects and demos:

- [accelerometer](https://github.com/GitJer/Some-Sipeed-MAIX-GO-k210-stuff/tree/master/src/accelerometer) - Example of using the MSA300 accelerometer on the MAIX Go board, by GitJer

- [quake1](https://github.com/elect-gombe/quake-k210) - Quake 1 on K210. Requires [wiring up a PS2 controller](https://robotzero.one/quake-kendryte-k210-risc-v/).

- [doom1](https://github.com/elect-gombe/k210-doom) - Doom 1 on K210
