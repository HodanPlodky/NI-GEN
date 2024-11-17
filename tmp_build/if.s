.global _start
_start:
    call main
    addi a7, zero, 93
    ecall
f:
    addi sp, sp, -8
    addi x28, zero, 1
    sd x28, 0(sp)
    addi x28, zero, 1
    beq x28, zero, f+36
    jal zero, f+24
    addi x28, zero, 2
    sd x28, 0(sp)
    jal zero, f+36
    ld x28, 0(sp)
    addi a0, x28, 0
    addi sp, sp, 8
    ret
g:
    addi sp, sp, -8
    addi x28, zero, 1
    beq x28, zero, g+28
    jal zero, g+16
    addi x28, zero, 5
    sd x28, 0(sp)
    jal zero, g+40
    addi x28, zero, 10
    sd x28, 0(sp)
    jal zero, g+40
    ld x28, 0(sp)
    addi a0, x28, 0
    addi sp, sp, 8
    ret
main:
    addi x28, zero, 1
    beq x28, zero, main+20
    jal zero, main+12
    addi a0, zero, 1
    ret
    addi a0, zero, 0
    ret
