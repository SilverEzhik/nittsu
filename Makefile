ARCH?=x86_64

default: run

build:
	bootimage build

run: build
	tmux resize-pane -Z
	qemu-system-$(ARCH) -drive format=raw,file=target/$(ARCH)-nittsu/debug/bootimage-nittsu.bin -curses
	tmux resize-pane -Z

dep:
	vim Cargo.toml
