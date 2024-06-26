use core::arch::asm;

pub fn print_stack_trace() {
    let mut fp: *const usize;

    unsafe {
        asm!("mv {}, fp", out(reg) fp );
    }

    println!("== Begin stack trace ==");
    while !fp.is_null() {
        let saved_ra;
        let saved_fp;
        unsafe {
            saved_ra = *fp.sub(1);
            saved_fp = *fp.sub(2);
        }

        println!("0x{:016x}, fp=0x{:016x}", saved_ra, saved_fp);

        fp = saved_fp as *const usize;
    }
    println!("== End stack trace ==");
}
