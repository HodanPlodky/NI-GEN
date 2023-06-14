.global _start

_start:
    call main

    addi a7, zero, 93
    ecall

main:
    addi a7, zero, 64
    addi a0, zero, 1 # stdout file descriptor = 1
    la a1, msg
    addi a2, zero, 10
    ecall

    li a0, 40
    
    mv s0, ra
    call fib
    mv ra, s0
    ret

fib:
    addi sp, sp, -24
    addi t0, zero, 1
    bge t0, a0, fib+34
    
    # store return address and parameter
    sd ra, 16(sp)
    sd a0, 8(sp)
    # recursion into n - 1
    addi a0, a0, -1
    call fib
    sd a0, 0(sp)
    # recursion into n - 2
    ld a0, 8(sp)
    addi a0, a0, -2
    call fib
    
    # sum
    ld t0, 0(sp)
    add a0, a0, t0

    ld ra, 16(sp)

    addi sp, sp, 24
    ret

msg:
    .ascii "Yo bitch\n"
