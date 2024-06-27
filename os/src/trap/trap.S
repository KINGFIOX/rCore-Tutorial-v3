.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm

.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm

    .section .text
    .globl __alltraps
    .globl __restore
    .align 2
__alltraps:
    csrrw sp, sscratch, sp
    # now sp->kernel stack, sscratch->user stack
    # allocate a TrapContext on kernel stack
    addi sp, sp, -34*8
    # save general-purpose registers
    sd x1, 1*8(sp)
    # skip sp(x2), we will save it later
    sd x3, 3*8(sp)
    # skip tp(x4), application does not use it
    # save x5~x31
    .set n, 5  // 5 + 27 = 32
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    # we can use t0/t1/t2 freely, because they were saved on kernel stack
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    # read user stack from sscratch and save it on the kernel stack
    csrr t2, sscratch
    sd t2, 2*8(sp)
    # set input argument of trap_handler(cx: &mut TrapContext)
    mv a0, sp
    call trap_handler

__restore:
    # 传进来的是 TrapContext 的指针
    mv sp, a0

    ld t0, 32*8(sp)  # sstatus
    csrw sstatus, t0
    ld t1, 33*8(sp)  # sepc -> entry
    csrw sepc, t1
    ld t2, 2*8(sp)  # sp -> 用户栈
    csrw sscratch, t2  # sscratch 
    ld x1, 1*8(sp)  # ra

    ld x3, 3*8(sp)  # gp 用于指向全局数据区

    # 恢复寄存器上下文
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr

    addi sp, sp, 34*8

    # now sp->kernel stack, sscratch->user stack
    csrrw sp, sscratch, sp
    sret # S -> U
    # sp -> 用户栈
    # pc <- sepc(entry)
