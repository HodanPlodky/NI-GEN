AS = riscv64-linux-gnu-as
CC = riscv64-linux-gnu-gcc
CFLAGS = -ggdb -fomit-frame-pointer -march=rv64g # -fno-pic

FILE=main

run: compile
	qemu-riscv64 -L /usr/riscv64-linux-gnu/ $(FILE)

compile:
	rm -rf $(FILE)
	$(CC) $(CFLAGS) -c $(FILE).s -o $(FILE).o
	$(CC) $(CFLAGS) -o $(FILE) $(FILE).o -nostdlib -static

gdb: compile
	qemu-riscv64 -L /usr/riscv64-linux-gnu/ -g 1234 $(FILE) &
	riscv64-linux-gnu-gdb -ex 'target remote localhost:1234' -ex 'file $(FILE)'

