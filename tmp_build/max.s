.global _start
_start:
    call main
    addi a7, zero, 93
    ecall
main:
    addi sp, sp, -48
    addi x28, zero, 7
    addi x7, zero, 18
    addi a0, zero, 10
    addi a1, x7, 0
    sd ra, 0(sp)
    sd x28, 24(sp)
    sd x7, 32(sp)
    sd x6, 40(sp)
    call max
    ld x6, 40(sp)
    ld x7, 32(sp)
    ld x28, 24(sp)
    ld ra, 0(sp)
    addi x6, a0, 0
    addi a0, zero, 2
    addi a1, x28, 0
    sd ra, 8(sp)
    sd x28, 24(sp)
    sd x6, 32(sp)
    sd x7, 40(sp)
    call max
    ld x7, 40(sp)
    ld x6, 32(sp)
    ld x28, 24(sp)
    ld ra, 8(sp)
    addi x7, a0, 0
    addi a0, x6, 0
    addi a1, x7, 0
    sd ra, 16(sp)
    sd x6, 24(sp)
    sd x7, 32(sp)
    call max
    ld x7, 32(sp)
    ld x6, 24(sp)
    ld ra, 16(sp)
    addi sp, sp, 48
    ret
max:
    addi x28, a0, 0
    addi x7, a1, 0
    bge x28, x7, max+24
    jal zero, max+16
    addi a0, x7, 0
    ret
    addi a0, x28, 0
    ret
    ret
