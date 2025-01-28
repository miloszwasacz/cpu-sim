main:
        sub     sp, sp, #64
        str     wzr, [sp, #60]
        mov     w8, #3
        str     w8, [sp, #20]
        mov     w8, #2
        str     w8, [sp, #24]
        mov     w8, #4
        str     w8, [sp, #28]
        mov     w8, #1
        str     w8, [sp, #32]
        mov     w8, #9
        str     w8, [sp, #36]
        mov     w8, #7
        str     w8, [sp, #40]
        mov     w8, #8
        str     w8, [sp, #44]
        mov     w8, #10
        str     w8, [sp, #48]
        mov     w9, #5
        str     w9, [sp, #52]
        mov     w9, #6
        str     w9, [sp, #56]
        str     w8, [sp, #16]
        b       .LBB0_1
.LBB0_1:
        ldr     w8, [sp, #16]
        subs    w8, w8, #1
        b.le    .LBB0_9
        b       .LBB0_2
.LBB0_2:
        str     wzr, [sp, #12]
        b       .LBB0_3
.LBB0_3:
        ldr     w8, [sp, #12]
        ldr     w9, [sp, #16]
        subs    w8, w8, w9
        b.ge    .LBB0_8
        b       .LBB0_4
.LBB0_4:
        ldrsw   x8, [sp, #12]
        add     x9, sp, #20
        ldr     w8, [x9, x8, lsl #2]
        ldr     w10, [sp, #12]
        add     w10, w10, #1
        ldr     w9, [x9, w10, sxtw #2]
        subs    w8, w8, w9
        b.le    .LBB0_6
        b       .LBB0_5
.LBB0_5:
        ldrsw   x8, [sp, #12]
        add     x9, sp, #20
        ldr     w8, [x9, x8, lsl #2]
        str     w8, [sp, #8]
        ldr     w8, [sp, #12]
        add     w8, w8, #1
        ldr     w8, [x9, w8, sxtw #2]
        ldrsw   x10, [sp, #12]
        str     w8, [x9, x10, lsl #2]
        ldr     w8, [sp, #8]
        ldr     w10, [sp, #12]
        add     w10, w10, #1
        str     w8, [x9, w10, sxtw #2]
        b       .LBB0_6
.LBB0_6:
        b       .LBB0_7
.LBB0_7:
        ldr     w8, [sp, #12]
        add     w8, w8, #1
        str     w8, [sp, #12]
        b       .LBB0_3
.LBB0_8:
        b       .LBB0_1
.LBB0_9:
        ldr     w0, [sp, #60]
        add     sp, sp, #64
        ret