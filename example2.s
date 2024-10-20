.global _start

_start:
    call main
    nop
    addi a7, zero, 93
    ecall

main:
    addi sp, sp, -4
    addi a7, zero, 64
    addi a0, zero, 1 # stdout file descriptor = 1
    la a1, msg
    addi a2, zero, 10
    ecall

    # write to stack
    addi t0, zero, 'h'
    sb t0, 0(sp)
    addi t0, zero, 'e'
    sb t0, 1(sp)
    addi t0, zero, 'l'
    sb t0, 2(sp)
    addi t0, zero, 'l'
    sb t0, 3(sp)
    addi t0, zero, 'o'
    sb t0, 4(sp)

    addi a0, zero, 1
    addi a1, sp, 0
    addi a2, zero, 5
    addi a7, zero, 64
    ecall


    li a0, 20
    
    ret

msg:
    .ascii "Yo bitch\n"

