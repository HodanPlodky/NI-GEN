.global _start
_start:
    addi sp, sp, 0
    call main
    addi a7, zero, 93
    ecall
main:
    addi sp, sp, -24
    addi x28, zero, 1
    addi x31, zero, 8
    mul x31, x31, x28
    addi x31, sp, 0
    add x31, x31, x31
    addi x7, x31, 0
    addi x28, zero, 1
    sd x28, 0(x7)
    addi x28, zero, 1
    addi x31, zero, 8
    mul x31, x31, x28
    addi x31, sp, 0
    add x31, x31, x31
    addi x7, x31, 0
    ld x28, 0(x7)
    addi a0, x28, 0
    addi sp, sp, 24
    ret
