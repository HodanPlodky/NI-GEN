.global _start
_start:
    call main
    addi a7, zero, 93
    ecall
main:
    addi sp, sp, -8
    addi a0, zero, 5
    sd ra, 0(sp)
    call fact
    ld ra, 0(sp)
    addi sp, sp, 8
    ret
fact:
    addi sp, sp, -16
    addi x28, a0, 0
    addi x31, zero, 2
    bge x28, x31, fact+32
    jal zero, fact+20
    addi a0, zero, 1
    addi sp, sp, 16
    ret
    addi a0, x28, -1
    sd ra, 0(sp)
    sd x28, 8(sp)
    call fact
    ld x28, 8(sp)
    ld ra, 0(sp)
    mul x7, x28, a0
    addi a0, x7, 0
    addi sp, sp, 16
    ret
