Maix Go / K210 stuff
=====================

Building the C projects
-----------------------

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

Building the Rust projects
--------------------------

Target configuration is set up in `.cargo/config`, so building is a matter of:

```
cd rust/<name_of_project>
cargo build --release
```

This will produce an ELF executable in the workspace's target directory named
`rust/target/riscv64gc-unknown-none-elf/release/<name_of_project>`.

If you have openocd working for the board, the below should work:
```
cargo run
```

Otherwise, see next section.

Running ELF
-----------

There is no need anymore to convert to raw binary, as ELF executables can be executed directly on
the device (no flashing) using

```bash
kflash.py -t -s -p /dev/ttyUSB1 -B goE "${ELF_NAME}"
```

This works for both the C and Rust-produced executables. It is also possible to upload
and run code on the device through JTAG and OpenOCD, but I have never got this to work myself
(openocd cannot find the device).

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

This is a general random sandbox with silly projects for me to play around with the Maix Go, some are in C and some are in Rust.

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

secp256k1\_{tests,bench}
------------------------

Run tests and benchmarks for the secp256k1 elliptic curve cryptographic library on this RISC-V CPU.

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

rust/game-of-life
-----------------

"Game of life" cellular automata simulation. The state can be manipulated through the touch screen.
The amount of pressure applied determines the radius of the state change.

[README](rust/game-of-life/README.md)

ROM re'ing
===========

Annotated radare2 config files for the Boot ROM and OTP can be found under [r2](r2/README.md).

Other projects
==============

Some interesting K210-based projects and demos by other people:

- [accelerometer](https://github.com/GitJer/Some-Sipeed-MAIX-GO-k210-stuff/tree/master/src/accelerometer) - Example of using the MSA300 accelerometer on the MAIX Go board, by GitJer

- [quake1](https://github.com/elect-gombe/quake-k210) - Quake 1 on K210. Requires [wiring up a PS2 controller](https://robotzero.one/quake-kendryte-k210-risc-v/).

- [doom1](https://github.com/elect-gombe/k210-doom) - Doom 1 on K210
