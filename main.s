.global _start
_start:
    call main
    addi a7, zero, 93
    ecall
f:
    addi x5, x0, 1
    addi x6, x0, 3
    add x7, x5, x6
    addi x10, x7, 0
    ret
main:
    addi x5, x0, 11
    addi x6, x0, 2
    add x7, x5, x6
    addi x10, x7, 0
    ret
