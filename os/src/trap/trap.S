    .altmacro
    .macro    SAVE_GP n
    sd        x\n, \n*8(sp)
    .endm
    .macro    LOAD_GP n
    ld        x\n, \n*8(sp)
    .endm
    .section  .text
    .globl    __alltraps
    .globl    __restore
    .align    2
__alltraps:
    csrrw     sp, sscratch, sp # sscratch = user_sp, sp = user_sp = context, 现在切换到内核栈了
# now sp->kernel stack, sscratch->user stack
# allocate a TrapContext on kernel stack
    addi      sp, sp, -34*8
# save general-purpose registers
    sd        x1, 1*8(sp)
# skip sp(x2), we will save it later
    sd        x3, 3*8(sp)
# skip tp(x4), application does not use it

# save x5~x31
    .set      n, 5
    .rept     27
    SAVE_GP   %n
    .set      n, n+1
    .endr

    csrr      t0, sstatus      # 进入 __alltrap 时刻的 sstatus
    csrr      t1, sepc         # 进入 __alltrap 时刻的 sepc
    sd        t0, 32*8(sp)     # user_sp
    sd        t1, 33*8(sp)     # user_pc
# read user stack from sscratch and save it on the kernel stack
    csrr      t2, sscratch     # user_sp
    sd        t2, 2*8(sp)
# set input argument of trap_handler(cx: &mut TrapContext)
    mv        a0, sp           #
    call      trap_handler     # 应该是 ret 到下面这个 __restore 这里

__restore:
# now sp->kernel stack(after allocated), sscratch->user stack
# restore sstatus/sepc
    ld        t0, 32*8(sp)
    ld        t1, 33*8(sp)
    ld        t2, 2*8(sp)
    csrw      sstatus, t0
    csrw      sepc, t1
    csrw      sscratch, t2
# restore general-purpuse registers except sp/tp
    ld        x1, 1*8(sp)
    ld        x3, 3*8(sp)
    .set      n, 5
    .rept     27
    LOAD_GP   %n
    .set      n, n+1
    .endr
# release TrapContext on kernel stack
    addi      sp, sp, 34*8
# now sp->kernel stack, sscratch->user stack
    csrrw     sp, sscratch, sp
    sret
