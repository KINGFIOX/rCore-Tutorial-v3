# 启用TUI模式
tui enable

# 显示反汇编视图
layout asm

layout regs

# 注意：GDB TUI可能不支持同时显示反汇编和寄存器视图
# 如果需要查看寄存器，可以使用以下命令手动切换
# tui reg general

# 在0x80200000处设置断点
break *0x80200000

break *0x80400000

# 自动运行程序，如果需要的话，取消下一行的注释
# run