.global _start
_start:
    call main
    addi a7, zero, 93
    ecall
main:
    addi a0, zero, 21
    ret
