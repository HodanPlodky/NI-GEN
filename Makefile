AS = riscv64-linux-gnu-as
CC = riscv64-linux-gnu-gcc
CFLAGS = -ggdb -fomit-frame-pointer # -fno-pic

run: compile
	qemu-riscv64 -L /usr/riscv64-linux-gnu/ example

compile:
	rm -rf example
	$(CC) $(CFLAGS) -c example.s -o example.o
	$(CC) $(CFLAGS) -o example example.o -nostdlib -static

gdb: compile
	qemu-riscv64 -L /usr/riscv64-linux-gnu/ -g 1234 example &
	riscv64-linux-gnu-gdb -ex 'target remote localhost:1234'

