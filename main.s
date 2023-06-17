.global _start
_start:
    call main
    addi a7, zero, 93
    ecall
main:
    addi x2, x2, -24
    addi x28, x0, 5
    sd x28, 0(x2)
    ld x7, 0(x2)
    addi x10, x7, 0
    sd x1, 8(x2)
    call inc
    ld x1, 8(x2)
    addi x6, x10, 0
    addi x5, x0, 2
    ld x31, 16(x2)
    add x31, x6, x5
    sd x31, 16(x2)
    ld x31, 16(x2)
    addi x10, x31, 0
    addi x2, x2, 24
    ret
inc:
    addi x2, x2, -8
    addi x28, x10, 0
    sd x28, 0(x2)
    ld x7, 0(x2)
    addi x6, x0, 1
    add x5, x7, x6
    addi x10, x5, 0
    addi x2, x2, 8
    ret
