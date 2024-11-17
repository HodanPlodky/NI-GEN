.global _start
_start:
    call main
    addi a7, zero, 93
    ecall
main:
    addi sp, sp, -104
    addi x28, zero, 0
    slli x7, x28, 3
    addi x31, sp, 0
    add x7, sp, x7
    addi x28, zero, 1
    sd x28, 0(x7)
    addi x28, zero, 1
    slli x7, x28, 3
    addi x31, sp, 0
    add x7, sp, x7
    addi x28, zero, 20
    sd x28, 0(x7)
    addi x28, zero, 2
    slli x7, x28, 3
    addi x31, sp, 0
    add x7, sp, x7
    addi x28, zero, 0
    slli x6, x28, 3
    addi x31, sp, 0
    add x6, sp, x6
    ld x28, 0(x6)
    addi x6, zero, 1
    slli x5, x6, 3
    addi x31, sp, 0
    add x5, sp, x5
    ld x6, 0(x5)
    add x5, x28, x6
    sd x5, 0(x7)
    addi x5, zero, 2
    slli x7, x5, 3
    addi x31, sp, 24
    add x7, x31, x7
    addi x5, zero, 39
    sd x5, 0(x7)
    addi x5, zero, 2
    slli x7, x5, 3
    addi x31, sp, 0
    add x7, sp, x7
    ld x5, 0(x7)
    addi x7, zero, 2
    slli x6, x7, 3
    addi x31, sp, 24
    add x6, x31, x6
    ld x7, 0(x6)
    add x6, x5, x7
    addi a0, x6, 0
    addi sp, sp, 104
    ret
