.global _start
_start:
    call main
    addi a7, zero, 93
    ecall
bubblesort:
    addi sp, sp, -80
    addi x28, a0, 0
    addi x7, a1, 0
    addi x6, zero, 0
    sd x6, 0(sp)
    jal zero, bubblesort+24
    ld x6, 0(sp)
    bge x6, x7, bubblesort+48
    jal zero, bubblesort+36
    addi x6, zero, 0
    sd x6, 8(sp)
    jal zero, bubblesort+56
    addi sp, sp, 80
    ret
    ld x6, 8(sp)
    addi x5, x7, -1
    ld x31, 0(sp)
    sd x31, 16(sp)
    sub x30, x5, x31
    sd x30, 24(sp)
    addi x31, x30, 0
    bge x6, x31, bubblesort+144
    jal zero, bubblesort+92
    ld x6, 8(sp)
    slli x5, x6, 3
    add x5, x28, x5
    ld x6, 0(x5)
    ld x5, 8(sp)
    addi x31, x5, 1
    sd x31, 32(sp)
    slli x5, x31, 3
    add x5, x28, x5
    ld x31, 0(x5)
    sd x31, 40(sp)
    bge x31, x6, bubblesort+260
    jal zero, bubblesort+160
    ld x6, 0(sp)
    addi x5, x6, 1
    sd x5, 0(sp)
    jal zero, bubblesort+24
    ld x5, 8(sp)
    slli x6, x5, 3
    add x6, x28, x6
    ld x5, 0(x6)
    ld x6, 8(sp)
    slli x31, x6, 3
    sd x31, 48(sp)
    add x30, x28, x31
    sd x30, 48(sp)
    ld x6, 8(sp)
    addi x31, x6, 1
    sd x31, 56(sp)
    slli x6, x31, 3
    add x6, x28, x6
    ld x31, 0(x6)
    sd x31, 64(sp)
    ld x30, 48(sp)
    sd x31, 0(x30)
    ld x6, 8(sp)
    addi x31, x6, 1
    sd x31, 72(sp)
    slli x6, x31, 3
    add x6, x28, x6
    sd x5, 0(x6)
    jal zero, bubblesort+260
    ld x6, 8(sp)
    addi x5, x6, 1
    sd x5, 8(sp)
    jal zero, bubblesort+56
main:
    addi sp, sp, -72
    addi x28, zero, 0
    slli x7, x28, 3
    addi x31, sp, 8
    add x7, x31, x7
    addi x28, zero, 1
    sd x28, 0(x7)
    addi x28, zero, 1
    slli x7, x28, 3
    addi x31, sp, 8
    add x7, x31, x7
    addi x28, zero, 5
    sd x28, 0(x7)
    addi x28, zero, 2
    slli x7, x28, 3
    addi x31, sp, 8
    add x7, x31, x7
    addi x28, zero, 3
    sd x28, 0(x7)
    addi x28, zero, 3
    slli x7, x28, 3
    addi x31, sp, 8
    add x7, x31, x7
    addi x28, zero, 2
    sd x28, 0(x7)
    addi x28, zero, 4
    slli x7, x28, 3
    addi x31, sp, 8
    add x7, x31, x7
    addi x28, zero, 4
    sd x28, 0(x7)
    addi x28, zero, 0
    sd x28, 48(sp)
    addi x28, zero, 5
    addi x31, sp, 8
    addi a0, sp, 8
    addi a1, x28, 0
    sd ra, 0(sp)
    sd x28, 64(sp)
    call bubblesort
    ld x28, 64(sp)
    ld ra, 0(sp)
    addi x28, zero, 0
    sd x28, 56(sp)
    jal zero, main+180
    ld x28, 56(sp)
    addi x7, zero, 5
    bge x28, x7, main+244
    jal zero, main+196
    ld x7, 56(sp)
    slli x28, x7, 3
    addi x31, sp, 8
    add x28, x31, x28
    ld x7, 0(x28)
    ld x28, 56(sp)
    addi x6, x28, 1
    sub x28, x7, x6
    sltiu x28, x28, 1
    sltiu x6, x28, 1
    beq x6, zero, main+316
    jal zero, main+300
    addi x6, zero, 0
    slli x28, x6, 3
    addi x31, sp, 8
    add x28, x31, x28
    ld x6, 48(sp)
    sd x6, 0(x28)
    addi x6, zero, 0
    slli x28, x6, 3
    addi x31, sp, 8
    add x28, x31, x28
    ld x6, 0(x28)
    addi a0, x6, 0
    addi sp, sp, 72
    ret
    ld x6, 56(sp)
    addi a0, x6, 0
    addi sp, sp, 72
    ret
    ld x6, 48(sp)
    ld x28, 56(sp)
    slli x7, x28, 3
    addi x31, sp, 8
    add x7, x31, x7
    ld x28, 0(x7)
    add x7, x6, x28
    sd x7, 48(sp)
    ld x7, 56(sp)
    addi x28, x7, 1
    sd x28, 56(sp)
    jal zero, main+180
