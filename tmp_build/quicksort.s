.global _start
_start:
    call main
    addi a7, zero, 93
    ecall
quicksort:
    addi sp, sp, -56
    addi x28, a0, 0
    addi x7, a1, 0
    addi x6, a2, 0
    bge x7, x6, quicksort+196
    jal zero, quicksort+24
    addi a0, x28, 0
    addi a1, x7, 0
    addi a2, x6, 0
    sd ra, 0(sp)
    sd x28, 24(sp)
    sd x7, 32(sp)
    sd x6, 40(sp)
    sd x5, 48(sp)
    call partition
    ld x5, 48(sp)
    ld x6, 40(sp)
    ld x7, 32(sp)
    ld x28, 24(sp)
    ld ra, 0(sp)
    addi x5, a0, 0
    addi a0, x28, 0
    addi a1, x7, 0
    addi a2, x5, 0
    sd ra, 8(sp)
    sd x28, 24(sp)
    sd x7, 32(sp)
    sd x6, 40(sp)
    sd x5, 48(sp)
    call quicksort
    ld x5, 48(sp)
    ld x6, 40(sp)
    ld x7, 32(sp)
    ld x28, 24(sp)
    ld ra, 8(sp)
    addi x7, x5, 1
    addi a0, x28, 0
    addi a1, x7, 0
    addi a2, x6, 0
    sd ra, 16(sp)
    sd x28, 24(sp)
    sd x6, 32(sp)
    sd x7, 40(sp)
    call quicksort
    ld x7, 40(sp)
    ld x6, 32(sp)
    ld x28, 24(sp)
    ld ra, 16(sp)
    jal zero, quicksort+196
    addi sp, sp, 56
    ret
partition:
    addi sp, sp, -136
    addi x28, a0, 0
    addi x7, a1, 0
    addi x6, a2, 0
    sd x7, 0(sp)
    slli x5, x7, 3
    add x5, x28, x5
    ld x31, 0(x5)
    sd x31, 8(sp)
    addi x5, x7, 1
    sd x5, 16(sp)
    jal zero, partition+48
    ld x5, 16(sp)
    bge x5, x6, partition+100
    jal zero, partition+60
    ld x5, 16(sp)
    slli x31, x5, 3
    sd x31, 24(sp)
    add x30, x28, x31
    sd x30, 24(sp)
    addi x31, x30, 0
    ld x5, 0(x31)
    ld x31, 8(sp)
    bge x5, x31, partition+368
    jal zero, partition+236
    ld x5, 0(sp)
    slli x31, x5, 3
    sd x31, 32(sp)
    add x30, x28, x31
    sd x30, 32(sp)
    addi x31, x30, 0
    ld x5, 0(x31)
    ld x31, 0(sp)
    sd x31, 40(sp)
    slli x30, x31, 3
    sd x30, 48(sp)
    addi x31, x30, 0
    add x30, x28, x30
    sd x30, 48(sp)
    slli x31, x7, 3
    sd x31, 56(sp)
    add x30, x28, x31
    sd x30, 56(sp)
    addi x31, x30, 0
    ld x30, 0(x31)
    sd x30, 64(sp)
    addi x31, x30, 0
    ld x30, 48(sp)
    sd x31, 0(x30)
    slli x31, x7, 3
    sd x31, 72(sp)
    add x30, x28, x31
    sd x30, 72(sp)
    addi x31, x30, 0
    sd x5, 0(x31)
    ld x5, 0(sp)
    addi a0, x5, 0
    addi sp, sp, 136
    ret
    ld x5, 0(sp)
    addi x31, x5, 1
    sd x31, 80(sp)
    sd x31, 0(sp)
    ld x31, 80(sp)
    slli x5, x31, 3
    add x5, x28, x5
    ld x31, 0(x5)
    sd x31, 88(sp)
    ld x31, 80(sp)
    slli x5, x31, 3
    add x5, x28, x5
    ld x31, 16(sp)
    sd x31, 96(sp)
    slli x30, x31, 3
    sd x30, 104(sp)
    addi x31, x30, 0
    add x30, x28, x30
    sd x30, 104(sp)
    addi x31, x30, 0
    ld x30, 0(x31)
    sd x30, 112(sp)
    addi x31, x30, 0
    sd x31, 0(x5)
    ld x5, 16(sp)
    slli x31, x5, 3
    sd x31, 120(sp)
    add x30, x28, x31
    sd x30, 120(sp)
    ld x31, 88(sp)
    ld x30, 120(sp)
    sd x31, 0(x30)
    jal zero, partition+368
    ld x5, 16(sp)
    addi x31, x5, 1
    sd x31, 128(sp)
    sd x31, 16(sp)
    jal zero, partition+48
main:
    addi sp, sp, -80
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
    addi x28, zero, 0
    addi x7, zero, 5
    addi x31, sp, 8
    addi a0, sp, 8
    addi a1, x28, 0
    addi a2, x7, 0
    sd ra, 0(sp)
    sd x28, 64(sp)
    sd x7, 72(sp)
    call quicksort
    ld x7, 72(sp)
    ld x28, 64(sp)
    ld ra, 0(sp)
    addi x7, zero, 0
    sd x7, 56(sp)
    jal zero, main+196
    ld x7, 56(sp)
    addi x28, zero, 5
    bge x7, x28, main+260
    jal zero, main+212
    ld x28, 56(sp)
    slli x7, x28, 3
    addi x31, sp, 8
    add x7, x31, x7
    ld x28, 0(x7)
    ld x7, 56(sp)
    addi x6, x7, 1
    sub x7, x28, x6
    sltiu x7, x7, 1
    sltiu x6, x7, 1
    beq x6, zero, main+332
    jal zero, main+316
    addi x6, zero, 0
    slli x7, x6, 3
    addi x31, sp, 8
    add x7, x31, x7
    ld x6, 48(sp)
    sd x6, 0(x7)
    addi x6, zero, 0
    slli x7, x6, 3
    addi x31, sp, 8
    add x7, x31, x7
    ld x6, 0(x7)
    addi a0, x6, 0
    addi sp, sp, 80
    ret
    ld x6, 56(sp)
    addi a0, x6, 0
    addi sp, sp, 80
    ret
    ld x6, 48(sp)
    ld x7, 56(sp)
    slli x28, x7, 3
    addi x31, sp, 8
    add x28, x31, x28
    ld x7, 0(x28)
    add x28, x6, x7
    sd x28, 48(sp)
    ld x28, 56(sp)
    addi x7, x28, 1
    sd x7, 56(sp)
    jal zero, main+196
