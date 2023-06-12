.global _start

_start:
    j main
main:
    addi a7, zero, 64
    addi a0, zero, 1 # stdout file descriptor = 1
    la a1, msg
    addi a2, zero, 10
    ecall

    addi a7, zero, 93
    addi a0, zero, 69
    ecall

msg:
    .ascii "Yo bitch\n"
