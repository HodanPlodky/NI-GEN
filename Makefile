AS = riscv64-linux-gnu-as
CC = riscv64-linux-gnu-gcc
CFLAGS = -ggdb -fomit-frame-pointer # -fno-pic

run: compile
	qemu-riscv64 -L /usr/riscv64-linux-gnu/ example

compile:
	rm -rf example
	$(AS) example.s -o example.o
	$(CC) -o example example.o -nostdlib -static

