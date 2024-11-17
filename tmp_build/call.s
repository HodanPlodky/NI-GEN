.global _start
_start:
    call main
    addi a7, zero, 93
    ecall
f:
    addi a0, zero, 3
    ret
main:
    addi sp, sp, -8
    sd ra, 0(sp)
    call f
    ld ra, 0(sp)
    addi sp, sp, 8
    ret
