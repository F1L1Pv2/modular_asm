lim data
sta r6    // Array start pointer
swa r7    // Inner loop counter 
swa zero
swa r8    // A value
swa zero
swa r9    // B value
lim (data + 16)
sta r10   // Array end pointer
swa zero
swa seg   // set SEG to 0

outerloop:
    innerloop:
        // load mem[r7] into r8
        ld r7
        swa r8

        // Increment r7
        swa r7
        addi 1
        swa r7

        // Load mem[r7] into r9
        ld r7
        sta r9

        // Compare r8 and r9 (r8 - r9)
        nand acc
        addi 1
        add.f r8

        // Skip the swap if r8 >= r9
        lim skipswap
        b carry

            // Write r8 into the higher address of the pair
            swa r8
            st r7
            swa r8

            // Decrement r7
            swa r7
            addi 0xF
            swa r7

            // Write r9 into the lower address of the pair
            swa r9
            st r7
            swa r9

            // Increment r7 to restore it
            swa r7
            addi 1
            swa r7

        skipswap:

        // Compare r7 and r10 (r7 - r10)
        lda r10
        nand acc
        addi 1
        add.f r7

        // Repeat the inner loop if r10 > r7
        lim innerloop
        b ncarry
    
    // Decrement r10
    swa r10
    addi.f 0xF
    swa r10

    // Restore r7 to array start
    lda r6
    swa r7

    lim outerloop
    b carry

// Stop
ext 0

db 0
data:
    db 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15





