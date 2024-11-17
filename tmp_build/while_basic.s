.global _start
_start:
    call main
    addi a7, zero, 93
    ecall
main:
    addi sp, sp, -24
    addi x28, zero, 0
    sd x28, 0(sp)
    addi x28, zero, 0
    sd x28, 8(sp)
    jal zero, main+24
    ld x28, 8(sp)
    addi x7, zero, 1000
    bge x28, x7, main+52
    jal zero, main+40
    addi x7, zero, 0
    sd x7, 16(sp)
    jal zero, main+68
    ld x7, 0(sp)
    addi a0, x7, 0
    addi sp, sp, 24
    ret
    ld x7, 16(sp)
    addi x28, zero, 1000
    bge x7, x28, main+124
    jal zero, main+84
    ld x28, 0(sp)
    ld x7, 8(sp)
    add x6, x28, x7
    ld x7, 16(sp)
    add x28, x6, x7
    sd x28, 0(sp)
    ld x28, 16(sp)
    addi x7, x28, 1
    sd x7, 16(sp)
    jal zero, main+68
    ld x7, 8(sp)
    addi x28, x7, 1
    sd x28, 8(sp)
    jal zero, main+24
