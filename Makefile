LD=ld
LDFLAGS=-m elf_i386 --entry=_start -nostdlib -T
RUSTC=rustc
RUSTCFLAGS=-O --target i386-intel-linux -Z no-landing-pads --emit=obj -o
NASM=nasm
NASMFLAGS=-f elf -o
QEMU=qemu-system-i386

.PHONY: clean

run: arch/x86/boot/floppy.img
	tmux split-window -h "$(QEMU) -fda $< -curses -monitor telnet:localhost:4444,server -s -S"
	tmux select-pane -L
	telnet localhost 4444

clean:
	rm -f arch/x86/boot/*.bin arch/x86/boot/*.o arch/x86/boot/*.img
	rm lib.o

kernel: arch/x86/boot/start.o lib.o
	$(LD) $(LDFLAGS) arch/x86/boot/new_linker.ld -o $@.elf $^

%.o: %.asm
	@echo ae
	$(NASM) $(NASMFLAGS) $@ $<

%.o: %.rs
	$(RUSTC) $(RUSTCFLAGS) $@ $<

