//! SBI call wrappers

/// use sbi call to putchar in console (qemu uart handler)
pub fn console_putchar(c: usize) {
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c);
}

/// use sbi call to set timer
pub fn set_timer(timer: usize) {
    // 通过 sbi 调用来设置计时器
    // 这个时间也是用: 系统运行了多久度量的
    sbi_rt::set_timer(timer as _);
}

/// use sbi call to shutdown the kernel
pub fn shutdown(failure: bool) -> ! {
    use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};
    if !failure {
        system_reset(Shutdown, NoReason);
    } else {
        system_reset(Shutdown, SystemFailure);
    }
    unreachable!()
}
