K210 Boot ROM re'ing
====================

This directory contains annotations (comments, function names, some
cross-referencing) for the K210 boot process. The [radare2](https://rada.re/r/)
reverse-engineering tool was used.

Where there were clear matches I've tried to use function names from the SDK. When not,
I've tried to think of an appropriate name. Some functions are unknown and still named
after the broad category `fcnXXXXXXXX._flash`, `fcnXXXXXXXX._otp`.

You need a dump of the K210 ROM (address 0x88000000..0x8801ffff) as `kendryte_rom.dat`
in the current directory.

To use the radare2 projects the straightforward way is to link them to the user projects
directory. I had no luck overriding `R2_RDATAHOME`.

```bash
ln -sf $PWD/k210_* $HOME/.local/share/radare2/projects
```

```bash
stat kendryte_rom.dat # must be 131072 bytes
r2 -p k210_rom
```

```bash
stat kendryte_otp.dat # must be 16384 bytes
r2 -p k210_otp
```
