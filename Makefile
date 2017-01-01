LD=ld
LDFLAGS=-m elf_i386 --entry=_start -nostdlib -T
RUSTC=rustc
RUSTCFLAGS=-O --target i686-unknown-linux-gnu -Z no-landing-pads --emit=obj -o
NASM=nasm
NASMFLAGS=-f elf -o
QEMU=qemu-system-i386

.PHONY: compile run clean disk.img

run: disk.img
	$(QEMU) -hda $< -curses

clean:
	rm -f arch/x86/boot/*.bin arch/x86/boot/*.o arch/x86/boot/*.img
	rm lib.o

kernel.elf: arch/x86/boot/start.o lib.o
	$(LD) $(LDFLAGS) arch/x86/boot/linker.ld -o $@ $^

disk.img: kernel.elf
	mount -o loop,offset=32256 disk.img /mnt
	mv kernel.elf /mnt/boot/
	umount /mnt

%.o: %.asm
	$(NASM) $(NASMFLAGS) $@ $<

%.o: %.rs
	$(RUSTC) $(RUSTCFLAGS) $@ $<

