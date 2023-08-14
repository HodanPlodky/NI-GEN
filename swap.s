swap:
    addi sp, sp, -24
    addi x28, a0, 0
    sd x28, 0(sp)
    addi x28, a1, 0
    sd x28, 8(sp)
    ld x28, 0(sp)
    ld x7, 0(x28)
    sd x7, 16(sp)
    ld x7, 0(sp)
    ld x28, 8(sp)
    ld x6, 0(x28)
    sd x6, 0(x7)
    ld x6, 8(sp)
    ld x7, 16(sp)
    sd x7, 0(x6)
    addi sp, sp, 24
    ret
