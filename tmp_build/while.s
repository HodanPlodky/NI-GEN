.global _start
_start:
    call main
    addi a7, zero, 93
    ecall
main:
    addi sp, sp, -32
    addi x28, zero, 0
    sd x28, 8(sp)
    addi x28, zero, 0
    sd x28, 16(sp)
    jal zero, main+24
    ld x28, 8(sp)
    addi x7, zero, 10
    bge x28, x7, main+96
    jal zero, main+40
    ld x7, 16(sp)
    ld x28, 8(sp)
    addi a0, x28, 10
    sd ra, 0(sp)
    sd x7, 24(sp)
    call betterfib
    ld x7, 24(sp)
    ld ra, 0(sp)
    add x28, x7, a0
    sd x28, 16(sp)
    ld x28, 8(sp)
    addi x7, x28, 1
    sd x7, 8(sp)
    jal zero, main+24
    ld x7, 16(sp)
    addi a0, x7, 0
    addi sp, sp, 32
    ret
fib:
    addi sp, sp, -32
    addi x28, a0, 0
    addi x31, zero, 1
    bge x28, x31, fib+32
    jal zero, fib+20
    addi a0, zero, 0
    addi sp, sp, 32
    ret
    addi x31, zero, 2
    bge x28, x31, fib+64
    jal zero, fib+52
    addi sp, sp, 32
    ret
    addi a0, zero, 1
    addi sp, sp, 32
    ret
    addi a0, x28, -1
    sd ra, 0(sp)
    sd x28, 16(sp)
    sd x7, 24(sp)
    call fib
    ld x7, 24(sp)
    ld x28, 16(sp)
    ld ra, 0(sp)
    addi x7, a0, 0
    addi a0, x28, -2
    sd ra, 8(sp)
    sd x7, 16(sp)
    call fib
    ld x7, 16(sp)
    ld ra, 8(sp)
    add x28, x7, a0
    addi a0, x28, 0
    addi sp, sp, 32
    ret
    jal zero, fib+44
betterfib:
    addi sp, sp, -32
    addi x28, a0, 0
    addi x7, zero, 0
    sd x7, 0(sp)
    addi x7, zero, 1
    sd x7, 8(sp)
    addi x7, zero, 0
    sd x7, 16(sp)
    jal zero, betterfib+36
    ld x7, 16(sp)
    bge x7, x28, betterfib+92
    jal zero, betterfib+48
    ld x7, 8(sp)
    ld x6, 0(sp)
    ld x5, 8(sp)
    add x31, x6, x5
    sd x31, 24(sp)
    sd x31, 8(sp)
    sd x7, 0(sp)
    ld x7, 16(sp)
    addi x5, x7, 1
    sd x5, 16(sp)
    jal zero, betterfib+36
    ld x28, 0(sp)
    addi a0, x28, 0
    addi sp, sp, 32
    ret
