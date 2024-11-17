.global _start
_start:
    call main
    addi a7, zero, 93
    ecall
main:
    addi sp, sp, -16
    addi x28, zero, 1
    sd x28, 8(sp)
    addi x28, zero, 10
    sd x28, 8(sp)
    addi x31, sp, 8
    addi a0, sp, 8
    sd ra, 0(sp)
    call f
    ld ra, 0(sp)
    ld x28, 8(sp)
    addi a0, x28, 0
    addi sp, sp, 16
    ret
f:
    addi x28, a0, 0
    ld x7, 0(x28)
    addi x6, x7, 1
    sd x6, 0(x28)
    ret
