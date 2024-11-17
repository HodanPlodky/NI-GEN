.global _start
_start:
    call main
    addi a7, zero, 93
    ecall
main:
    addi sp, sp, -8
    addi a0, zero, 40
    sd ra, 0(sp)
    call fib
    ld ra, 0(sp)
    addi sp, sp, 8
    ret
fib:
    addi sp, sp, -32
    addi x28, a0, 0
    addi x7, zero, 0
    sd x7, 0(sp)
    addi x7, zero, 1
    sd x7, 8(sp)
    addi x7, zero, 0
    sd x7, 16(sp)
    jal zero, fib+36
    ld x7, 16(sp)
    bge x7, x28, fib+92
    jal zero, fib+48
    ld x7, 8(sp)
    ld x6, 0(sp)
    ld x5, 8(sp)
    add x31, x6, x5
    sd x31, 24(sp)
    sd x31, 8(sp)
    sd x7, 0(sp)
    ld x7, 16(sp)
    addi x5, x7, 1
    sd x5, 16(sp)
    jal zero, fib+36
    ld x28, 0(sp)
    addi a0, x28, 0
    addi sp, sp, 32
    ret
