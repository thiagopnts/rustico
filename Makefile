CC=i386-elf-gcc
#using binutils installed with homebrew, remember to pass --target=i386-elf to ./configure
LD=/usr/local/Cellar/binutils/2.24/i386-elf/bin/ld
RUSTC=rustc
NASM=nasm
QEMU=qemu-system-i386

all: arch/x86/boot/floppy.img

.SUFFIXES:

.SUFFIXES: .o .rs .asm

.PHONY: clean run

.rs.o:
	$(RUSTC) -O --target i386-intel-linux -Z no-landing-pads --crate-type lib --emit=obj -o $@ $< 

.asm.o:
	$(NASM) -f elf32 -o $@ $<

arch/x86/boot/floppy.img: arch/x86/boot/loader.bin arch/x86/boot/main.bin
	cat $^ > $@

arch/x86/boot/loader.bin: arch/x86/boot/loader.asm
	$(NASM) -o $@ -f bin $<

arch/x86/boot/main.bin: arch/x86/boot/linker.ld arch/x86/boot/runtime.o lib.o
	$(LD) -o $@ -T $^

run: arch/x86/boot/floppy.img
	$(QEMU) -fda $<

clean:
	rm -f arch/x86/boot/*.bin arch/x86/boot/*.o arch/x86/boot/*.img
