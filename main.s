.global _start
_start:
    call main
    addi a7, zero, 93
    ecall
main:
    andi x2, x2, -56
    addi x28, x0, 1
    addi x7, x0, 2
    add x6, x28, x7
    addi x5, x0, 3
    ld x31, 0(x2)
    add x31, x6, x5
    sd x31, 0(x2)
    ld x31, 8(x2)
    addi x31, x0, 4
    sd x31, 8(x2)
    ld x31, 16(x2)
    ld x30, 0(x2)
    ld x29, 8(x2)
    add x31, x30, x29
    sd x31, 16(x2)
    ld x31, 24(x2)
    addi x31, x0, 5
    sd x31, 24(x2)
    ld x31, 32(x2)
    ld x30, 16(x2)
    ld x29, 24(x2)
    add x31, x30, x29
    sd x31, 32(x2)
    ld x31, 40(x2)
    addi x31, x0, 6
    sd x31, 40(x2)
    ld x31, 48(x2)
    ld x30, 32(x2)
    ld x29, 40(x2)
    add x31, x30, x29
    sd x31, 48(x2)
    ld x31, 48(x2)
    addi x10, x31, 0
    andi x2, x2, 56
    ret
