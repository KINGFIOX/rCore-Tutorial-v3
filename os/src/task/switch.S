    .altmacro
    .macro    SAVE_SN n
    sd        s\n, (\n+2)*8(a0)
    .endm

    .macro    LOAD_SN n
    ld        s\n, (\n+2)*8(a1)
    .endm

# pub struct TaskContext {
# /// return address ( e.g. __restore ) of __switch ASM function
# ra: usize,
# /// kernel stack pointer of app
# sp: usize,
# /// callee saved registers: s 0..11
# s: [usize; 12],
# }
    .section  .text
    .globl    __switch
__switch:
# __switch(
# current_task_cx_ptr: *mut TaskContext,
# next_task_cx_ptr: *const TaskContext
# )
# save kernel stack of current task
    sd        sp, 8(a0)
# save ra & s0~s11 of current execution
    sd        ra, 0(a0)         # 这里 store ra <- __switch's caller
    .set      n, 0
    .rept     12
    SAVE_SN   %n
    .set      n, n + 1
    .endr
# restore ra & s0~s11 of next execution
    ld        ra, 0(a1)
    .set      n, 0
    .rept     12
    LOAD_SN   %n
    .set      n, n + 1
    .endr
# restore kernel stack of next task
    ld        sp, 8(a1)
    ret

