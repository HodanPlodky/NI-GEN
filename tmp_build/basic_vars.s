.global _start
_start:
    call main
    addi a7, zero, 93
    ecall
main:
    addi sp, sp, -8
    addi a0, zero, 5
    sd ra, 0(sp)
    call inc
    ld ra, 0(sp)
    addi a0, a0, 2
    addi sp, sp, 8
    ret
inc:
    addi a0, a0, 1
    ret
