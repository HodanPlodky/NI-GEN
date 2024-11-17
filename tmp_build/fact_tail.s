.global _start
_start:
    call main
    addi a7, zero, 93
    ecall
fact:
    addi sp, sp, -24
    addi x28, a0, 0
    addi x7, zero, 1
    addi a0, x28, 0
    addi a1, x7, 0
    sd ra, 0(sp)
    sd x28, 8(sp)
    sd x7, 16(sp)
    call fact_inner
    ld x7, 16(sp)
    ld x28, 8(sp)
    ld ra, 0(sp)
    addi sp, sp, 24
    ret
main:
    addi sp, sp, -8
    addi a0, zero, 5
    sd ra, 0(sp)
    call fact
    ld ra, 0(sp)
    addi sp, sp, 8
    ret
fact_inner:
    addi sp, sp, -24
    addi x28, a0, 0
    addi x7, a1, 0
    addi x31, zero, 2
    bge x28, x31, fact_inner+36
    jal zero, fact_inner+24
    addi a0, x7, 0
    addi sp, sp, 24
    ret
    addi x6, x28, -1
    mul x5, x7, x28
    addi a0, x6, 0
    addi a1, x5, 0
    sd ra, 0(sp)
    sd x6, 8(sp)
    sd x5, 16(sp)
    call fact_inner
    ld x5, 16(sp)
    ld x6, 8(sp)
    ld ra, 0(sp)
    addi sp, sp, 24
    ret
