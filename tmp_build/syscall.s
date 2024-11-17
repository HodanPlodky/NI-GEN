.global _start
_start:
    call main
    addi a7, zero, 93
    ecall
main:
    addi sp, sp, -14
    addi x28, zero, 0
    slli x7, x28, 0
    addi x31, sp, 8
    add x7, x31, x7
    addi x28, zero, 104
    sb x28, 0(x7)
    addi x28, zero, 1
    slli x7, x28, 0
    addi x31, sp, 8
    add x7, x31, x7
    addi x28, zero, 101
    sb x28, 0(x7)
    addi x28, zero, 2
    slli x7, x28, 0
    addi x31, sp, 8
    add x7, x31, x7
    addi x28, zero, 108
    sb x28, 0(x7)
    addi x28, zero, 3
    slli x7, x28, 0
    addi x31, sp, 8
    add x7, x31, x7
    addi x28, zero, 108
    sb x28, 0(x7)
    addi x28, zero, 4
    slli x7, x28, 0
    addi x31, sp, 8
    add x7, x31, x7
    addi x28, zero, 111
    sb x28, 0(x7)
    addi x28, zero, 5
    slli x7, x28, 0
    addi x31, sp, 8
    add x7, x31, x7
    addi x28, zero, 10
    sb x28, 0(x7)
    addi x28, zero, 1
    addi x7, zero, 6
    addi a0, x28, 0
    addi x31, sp, 8
    addi a1, sp, 8
    addi a2, x7, 0
    addi a7, zero, 64
    ecall
    addi a0, zero, 123
    sd ra, 0(sp)
    call exit
    ld ra, 0(sp)
    addi a0, zero, 0
    addi sp, sp, 14
    ret
exit:
    addi a7, zero, 93
    ecall
    ret
