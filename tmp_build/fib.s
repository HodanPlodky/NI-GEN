.global _start
_start:
    call main
    addi a7, zero, 93
    ecall
fib:
    addi sp, sp, -32
    addi x28, a0, 0
    addi x31, zero, 2
    bge x28, x31, fib+32
    jal zero, fib+20
    addi a0, x28, 0
    addi sp, sp, 32
    ret
    addi a0, x28, -1
    sd ra, 0(sp)
    sd x28, 16(sp)
    sd x7, 24(sp)
    call fib
    ld x7, 24(sp)
    ld x28, 16(sp)
    ld ra, 0(sp)
    addi x7, a0, 0
    addi a0, x28, -2
    sd ra, 8(sp)
    sd x7, 16(sp)
    call fib
    ld x7, 16(sp)
    ld ra, 8(sp)
    add x28, x7, a0
    addi a0, x28, 0
    addi sp, sp, 32
    ret
main:
    addi sp, sp, -8
    addi a0, zero, 40
    sd ra, 0(sp)
    call fib
    ld ra, 0(sp)
    addi sp, sp, 8
    ret
