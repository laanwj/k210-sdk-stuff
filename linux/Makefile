CROSS ?= /opt/riscv64-uclibc/bin/riscv64-buildroot-linux-uclibc-
CFLAGS = -fPIC -Wl,-elf2flt=-r -Wall -Os -g
BINARIES = term esptun

all: $(BINARIES)

clean:
	rm -f $(BINARIES)

term: term.c
	${CROSS}gcc $< -o $@ $(CFLAGS)

esptun: esptun.c
	${CROSS}gcc $< -o $@ $(CFLAGS)

