lim (dest & 0xFF)
sta r6            // r6 - lower destination pointer
lim (dest >> 8)
sta r7            // r7 - upper destination pointer

lim source
sta r8            // r8 - lower source character pointer

strcpy:
    swa zero
    swa seg
    ld.f r8
    mov seg, r7
    st r6

    swa r6
    addi 1
    swa r6

    swa r8
    addi 1
    swa r8

    brc nzero, strcpy

ext 0

source: db "bajojajo\0"

org 512
dest: 
