default: run

.PHONY: default build run clean

target/multiboot_header.o: src/boot/x86_64/multiboot_header.asm
		mkdir -p target
		nasm -f elf64 src/boot/x86_64/multiboot_header.asm -o target/multiboot_header.o

target/boot.o: src/boot/x86_64/boot.asm
		mkdir -p target
		nasm -f elf64 src/boot/x86_64/boot.asm -o target/boot.o

target/kernel.bin: target/multiboot_header.o target/boot.o src/boot/x86_64/linker.ld cargo
		x86_64-pc-elf-ld -n -o target/kernel.bin -T src/boot/x86_64/linker.ld target/multiboot_header.o target/boot.o target/x86_64-nittsu/release/libnittsu.a

target/os.iso: target/kernel.bin src/boot/x86_64/grub.cfg
		mkdir -p target/isofiles/boot/grub
		cp src/boot/x86_64/grub.cfg target/isofiles/boot/grub
		cp target/kernel.bin target/isofiles/boot/
		grub-mkrescue -o target/os.iso target/isofiles

run: target/os.iso
		tmux resize-pane -Z
		qemu-system-x86_64 -cdrom target/os.iso -curses
		tmux resize-pane -Z

build: target/os.iso

cargo:
		# https://github.com/japaric/xargo/issues/44
		@RUST_TARGET_PATH=$(shell pwd) xargo build --release --target=x86_64-nittsu

clean:
		cargo clean
