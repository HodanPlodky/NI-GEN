.global _start
_start:
    call main
    addi a7, zero, 93
    ecall
main:
    addi x2, x2, -112
    addi x28, x0, 0
    sd x28, 0(x2)
    addi x7, x0, 0
    sd x7, 8(x2)
    jal x0, main+24
    ld x6, 0(x2)
    addi x5, x0, 10
    slt x31, x6, x5
    sd x31, 16(x2)
    ld x31, 16(x2)
    beq x31, x0, main+184
    ld x31, 8(x2)
    sd x31, 24(x2)
    ld x31, 0(x2)
    sd x31, 32(x2)
    addi x31, x0, 10
    sd x31, 40(x2)
    ld x30, 32(x2)
    ld x29, 40(x2)
    add x31, x30, x29
    sd x31, 48(x2)
    ld x31, 48(x2)
    addi x10, x31, 0
    sd x1, 56(x2)
    call fib
    ld x1, 56(x2)
    addi x31, x10, 0
    sd x31, 64(x2)
    ld x30, 24(x2)
    ld x29, 64(x2)
    add x31, x30, x29
    sd x31, 72(x2)
    ld x31, 72(x2)
    sd x31, 8(x2)
    ld x31, 0(x2)
    sd x31, 80(x2)
    addi x31, x0, 1
    sd x31, 88(x2)
    ld x30, 80(x2)
    ld x29, 88(x2)
    add x31, x30, x29
    sd x31, 96(x2)
    ld x31, 96(x2)
    sd x31, 0(x2)
    jal x0, main+24
    ld x31, 8(x2)
    sd x31, 104(x2)
    addi x10, x31, 0
    addi x2, x2, 112
    ret
fib:
    addi x2, x2, -136
    addi x28, x10, 0
    sd x28, 0(x2)
    ld x7, 0(x2)
    addi x6, x0, 0
    addi x5, x6, 1
    slt x5, x7, x5
    beq x5, x0, fib+52
    addi x31, x0, 0
    sd x31, 8(x2)
    addi x10, x31, 0
    addi x2, x2, 136
    ret
    ld x31, 0(x2)
    sd x31, 16(x2)
    addi x31, x0, 1
    sd x31, 24(x2)
    ld x30, 16(x2)
    ld x29, 24(x2)
    addi x31, x29, 1
    slt x31, x30, x31
    sd x31, 32(x2)
    ld x31, 32(x2)
    beq x31, x0, fib+124
    addi x2, x2, 136
    ret
    addi x31, x0, 1
    sd x31, 40(x2)
    addi x10, x31, 0
    addi x2, x2, 136
    ret
    ld x31, 0(x2)
    sd x31, 48(x2)
    addi x31, x0, 1
    sd x31, 56(x2)
    ld x30, 48(x2)
    ld x29, 56(x2)
    sub x31, x30, x29
    sd x31, 64(x2)
    ld x31, 64(x2)
    addi x10, x31, 0
    sd x1, 72(x2)
    call fib
    ld x1, 72(x2)
    addi x31, x10, 0
    sd x31, 80(x2)
    ld x31, 0(x2)
    sd x31, 88(x2)
    addi x31, x0, 2
    sd x31, 96(x2)
    ld x30, 88(x2)
    ld x29, 96(x2)
    sub x31, x30, x29
    sd x31, 104(x2)
    ld x31, 104(x2)
    addi x10, x31, 0
    sd x1, 112(x2)
    call fib
    ld x1, 112(x2)
    addi x31, x10, 0
    sd x31, 120(x2)
    ld x30, 80(x2)
    ld x29, 120(x2)
    add x31, x30, x29
    sd x31, 128(x2)
    addi x10, x31, 0
    addi x2, x2, 136
    ret
    jal x0, fib+96
