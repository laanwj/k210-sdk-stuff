Maix Go / K210 stuff
=====================

Some demo projects (mostly in rust) for the Maix Go.

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
address `0x80000000` in SRAM and directly executed.

Building the Rust projects
--------------------------

**Note:** *it's possible that these projects require Rust nightly to build.  I
don't intentially use nightly features, however, I always test only using the
latest one so it's likely that something will sneak in*

Make sure the appropriate target has been added to the toolchain that you wish
to use:
```bash
rustup target add riscv64gc-unknown-none-elf
```

Target configuration is set up in `.cargo/config`, so building is a matter of:

```bash
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
the device (without flashing) using a recent checkout of [kflash](https://github.com/kendryte/kflash.py)

```bash
kflash.py -t -s -p /dev/ttyUSB1 -B goE "${ELF_NAME}"
```

This works for both the C and Rust-produced executables. It is also possible to upload
and run code on the device through JTAG and OpenOCD, but I have never got this to work myself
(openocd cannot find the device).

Currently, rust generates ELF executables based at address `0xffffffff80000000`
instead of the expected `0x80000000`, to work around lack of medany memory
model support in LLVM (this has ben fixed but hasn't reached stable yet at the
time of writing). To make this work with kflash I had to patch the
following:

```patch
diff --git a/kflash.py b/kflash.py
index c092d08..b3bc457 100755
--- a/kflash.py
+++ b/kflash.py
@@ -976,7 +976,7 @@ class KFlash:
                     if segment['p_type']!='PT_LOAD' or segment['p_filesz']==0 or segment['p_vaddr']==0:
                         print("Skipped")
                         continue
-                    self.flash_dataframe(segment.data(), segment['p_vaddr'])
+                    self.flash_dataframe(segment.data(), segment['p_vaddr'] & 0xffffffff)

             def flash_firmware(self, firmware_bin, aes_key = None, address_offset = 0, sha256Prefix = True):
                 # type: (bytes, bytes, int, bool) -> None
```

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
It turns out that this cheap board is great for playing around with Rust embedded in an environment that
has a fair amount of memory and number of peripherals available by default!

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

rust/accelerometer
-----------------

Read measurements from MSA300 accelerometer. Display a dot on the screen to visualize the current orientation
and magnitude.

[README](rust/accelerometer/README.md)

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

rust/uart-passthrough
---------------------

Pass through UART from host to the ESP8285 WIFI chip.

[README](rust/uart-passthrough/README.md)

rust/rgbcontrol
----------------

Control the color of the RGB LED from the touch screen.

[README](rust/rgbcontrol/README.md)

rust/esp8266at
--------------

A crate for communicating with WiFi using the ESP8266 using AT commands. TODO: move this to its own repository.

[README](rust/esp8266at/README.md)

rust/weather
------------

Uses the ESP8285 WiFi chip of the Maix Go to fetch weather data from
[wttr.in](https://wttr.in) and print it to the display using `k210-console`.

[README](rust/weather/README.md)

rust/dvp-ov
-----------

A straightforward passthrough test for video handling, based on `dvp_ov` in the
SDK: read frames from the OV2640 image sensor and display them on the LCD.

[README](rust/dvp-ov/README.md)

rust/glyph-mapping
------------------

Rust port of the glyph mapping demo.

[README](rust/glyph-mapping/README.md)

rust/term-server
----------------

Uses the ESP8285 WiFi chip of the Maix Go to listen for incoming connections,
displaying the data on the terminal.

[README](rust/term-server/README.md)

rust/secp256k1-test
-------------------

Test for using the elliptic curve cryptography library `secp256k1`, written in C,
from rust on a RISC-V device.

[README](rust/secp256k1-test/README.md)

rust/sdtest
-----------

Read and write to a SD card using SPI.

[README](rust/sdtest/README.md)

rust/emdgfx
-----------

Experiments with `embedded-graphics` crate.

[README](rust/embgfx/README.md)

rust/voxel
-------------

Old-school voxel-based landscape renderer.

[README](rust/voxel/README.md)

rust/cryptest
-------------

Test the cryptographic acceleration engines of the K210.

[README](rust/cryptest/README.md)

rust/interrupt
--------------

Test for interrupts and use of the MMU.

ROM re'ing
===========

Annotated radare2 config files for the Boot ROM and OTP can be found under [r2](r2/README.md).

Other projects
==============

Some interesting K210-based projects and demos by other people:

- [accelerometer](https://github.com/GitJer/Some-Sipeed-MAIX-GO-k210-stuff/tree/master/src/accelerometer) - Example of using the MSA300 accelerometer on the MAIX Go board, by GitJer

- [quake1](https://github.com/elect-gombe/quake-k210) - Quake 1 on K210. Requires [wiring up a PS2 controller](https://robotzero.one/quake-kendryte-k210-risc-v/).

- [doom1](https://github.com/elect-gombe/k210-doom) - Doom 1 on K210
