CC=i386-elf-gcc
#using binutils installed with homebrew, remember to pass --target=i386-elf to ./configure
LD=/usr/local/Cellar/binutils/2.24/i386-elf/bin/ld
RUSTC=rustc
NASM=nasm
QEMU=qemu-system-i386

all: floppy.img

.SUFFIXES:

.SUFFIXES: .o .rs .asm

.PHONY: clean run

.rs.o:
	$(RUSTC) -O --target i386-intel-linux --crate-type lib --emit=obj -o $@ $<

.asm.o:
	$(NASM) -f elf32 -o $@ $<

main.rs: zero.rs

floppy.img: loader.bin main.bin
	cat $^ > $@

loader.bin: loader.asm
	$(NASM) -o $@ -f bin $<

main.bin: linker.ld runtime.o main.o
	$(LD) -o $@ -T $^

run: floppy.img
	$(QEMU) -fda $<

clean:
	rm -f *.bin *.o *.img
