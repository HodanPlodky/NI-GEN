.global _start
_start:
    call main
    addi a7, zero, 93
    ecall
main:
    addi x2, x2, -80
    addi x28, x0, 0
    sd x28, 0(x2)
    addi x7, x0, 0
    sd x7, 8(x2)
    jal x0, main+24
    ld x6, 0(x2)
    addi x5, x0, 10
    ld x31, 16(x2)
    slt x31, x6, x5
    sd x31, 16(x2)
    ld x31, 16(x2)
    beq x31, x0, main+152
    ld x31, 24(x2)
    ld x31, 8(x2)
    sd x31, 24(x2)
    ld x31, 32(x2)
    ld x31, 0(x2)
    sd x31, 32(x2)
    ld x31, 40(x2)
    ld x30, 24(x2)
    ld x29, 32(x2)
    add x31, x30, x29
    sd x31, 40(x2)
    ld x31, 40(x2)
    sd x31, 8(x2)
    ld x30, 48(x2)
    ld x30, 0(x2)
    sd x30, 48(x2)
    ld x31, 56(x2)
    addi x31, x0, 1
    sd x31, 56(x2)
    ld x31, 64(x2)
    ld x30, 48(x2)
    ld x29, 56(x2)
    add x31, x30, x29
    sd x31, 64(x2)
    jal x0, main+24
    ld x31, 72(x2)
    ld x31, 8(x2)
    sd x31, 72(x2)
    ld x31, 72(x2)
    addi x10, x31, 0
    addi x2, x2, 80
    ret
