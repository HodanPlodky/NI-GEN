.global _start
_start:
    call main
    addi a7, zero, 93
    ecall
main:
    addi sp, sp, -24
    addi x28, zero, 1
    sd x28, 8(sp)
    addi x28, zero, 2
    sd x28, 16(sp)
    addi x31, sp, 8
    addi a0, sp, 8
    addi x31, sp, 16
    addi a1, sp, 16
    sd ra, 0(sp)
    call swap
    ld ra, 0(sp)
    addi x28, zero, 3
    ld x7, 8(sp)
    mul x6, x28, x7
    ld x7, 16(sp)
    add x28, x6, x7
    addi a0, x28, 0
    addi sp, sp, 24
    ret
swap:
    addi x28, a0, 0
    addi x7, a1, 0
    ld x6, 0(x28)
    ld x5, 0(x7)
    sd x5, 0(x28)
    sd x6, 0(x7)
    ret
