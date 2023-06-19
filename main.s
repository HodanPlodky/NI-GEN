.global _start
_start:
    call main
    addi a7, zero, 93
    ecall
main:
    addi x2, x2, -8
    addi x28, x0, 5
    addi x10, x28, 0
    sd x1, 0(x2)
    call fib
    ld x1, 0(x2)
    addi x7, x10, 0
    addi x10, x7, 0
    addi x2, x2, 8
    ret
fib:
    addi x2, x2, -104
    addi x28, x10, 0
    sd x28, 0(x2)
    ld x7, 0(x2)
    addi x6, x0, 1
    addi x5, x7, 1
    slt x5, x5, x6
    beq x5, x0, fib+60
    ld x31, 8(x2)
    ld x31, 0(x2)
    sd x31, 8(x2)
    ld x31, 8(x2)
    addi x10, x31, 0
    addi x2, x2, 104
    ret
    ld x31, 16(x2)
    ld x31, 0(x2)
    sd x31, 16(x2)
    ld x31, 24(x2)
    addi x31, x0, 1
    sd x31, 24(x2)
    ld x31, 32(x2)
    ld x30, 16(x2)
    ld x29, 24(x2)
    sub x31, x30, x29
    sd x31, 32(x2)
    ld x31, 32(x2)
    addi x10, x31, 0
    sd x1, 40(x2)
    call fib
    ld x1, 40(x2)
    ld x31, 48(x2)
    addi x31, x10, 0
    sd x31, 48(x2)
    ld x31, 56(x2)
    ld x31, 0(x2)
    sd x31, 56(x2)
    ld x31, 64(x2)
    addi x31, x0, 2
    sd x31, 64(x2)
    ld x31, 72(x2)
    ld x30, 56(x2)
    ld x29, 64(x2)
    sub x31, x30, x29
    sd x31, 72(x2)
    ld x31, 72(x2)
    addi x10, x31, 0
    sd x1, 80(x2)
    call fib
    ld x1, 80(x2)
    ld x31, 88(x2)
    addi x31, x10, 0
    sd x31, 88(x2)
    ld x31, 96(x2)
    ld x30, 48(x2)
    ld x29, 88(x2)
    add x31, x30, x29
    sd x31, 96(x2)
    ld x31, 96(x2)
    addi x10, x31, 0
    addi x2, x2, 104
    ret
