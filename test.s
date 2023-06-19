.global _start

_start:
    call main
    li a7, 93
    ecall

main:
    li t0, 9
    li t1, 8
    beq a0, t1, t0
    slt a0, t1, t0
    ret

    
