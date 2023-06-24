.global _start
_start:
    call main
    addi a7, zero, 93
    ecall
main:
    addi x2, x2, -32
    addi x28, x0, 1
    sd x28, 0(x2)
    addi x31, x2, 0
    sd x31, 8(x2)
    ld x7, 8(x2)
    addi x6, x0, 10
    sd x6, 0(x7)
    addi x31, x2, 0
    addi x10, x31, 0
    sd x1, 16(x2)
    call f
    ld x1, 16(x2)
    addi x5, x10, 0
    ld x31, 0(x2)
    sd x31, 24(x2)
    addi x10, x31, 0
    addi x2, x2, 32
    ret
f:
    addi x2, x2, -24
    addi x28, x10, 0
    sd x28, 0(x2)
    ld x7, 0(x2)
    ld x6, 0(x2)
    ld x5, 0(x6)
    addi x31, x0, 1
    sd x31, 8(x2)
    ld x30, 8(x2)
    add x31, x5, x30
    sd x31, 16(x2)
    ld x31, 16(x2)
    sd x31, 0(x7)
    addi x2, x2, 24
    ret
