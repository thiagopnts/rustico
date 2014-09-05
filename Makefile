LD=ld
LDFLAGS=-m elf_i386 --entry=_start -nostdlib -T
RUSTC=rustc
RUSTCFLAGS=-O --target i386-intel-linux -Z no-landing-pads --emit=obj -o
NASM=nasm
NASMFLAGS=-f elf -o
QEMU=qemu-system-i386

.PHONY: run clean

run: disk.img
	tmux split-window -h "$(QEMU) -hda $< -curses -monitor telnet:localhost:4444,server -s -S"
	tmux select-pane -L

clean:
	rm -f arch/x86/boot/*.bin arch/x86/boot/*.o arch/x86/boot/*.img
	rm lib.o

kernel: arch/x86/boot/start.o lib.o
	$(LD) $(LDFLAGS) arch/x86/boot/new_linker.ld -o $@.elf $^

disk.img: kernel
	sudo mount -o loop,offset=32256 disk.img /mnt
	sudo mv kernel.elf /mnt/
	sudo umount /mnt

%.o: %.asm
	$(NASM) $(NASMFLAGS) $@ $<

%.o: %.rs
	$(RUSTC) $(RUSTCFLAGS) $@ $<

