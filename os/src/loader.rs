//! Loading user applications into memory
//!
//! For chapter 3, user applications are simply part of the data included in the
//! kernel binary, so we only need to copy them to the space allocated for each
//! app to load them. We also allocate fixed spaces for each task's
//! [`KernelStack`] and [`UserStack`].

use crate::config::*;
use crate::trap::TrapContext;
use core::arch::asm;

/* ---------- ---------- kernel ---------- ---------- */

#[repr(align(4096))]
#[derive(Copy, Clone)]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
    pub fn push_context(&self, trap_cx: TrapContext) -> usize {
        let trap_cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *trap_cx_ptr = trap_cx;
        }
        trap_cx_ptr as usize
    }
}

static KERNEL_STACK: [KernelStack; MAX_APP_NUM] = [KernelStack {
    data: [0; KERNEL_STACK_SIZE], // kernel stack 初始化为 0
}; MAX_APP_NUM];

/* ---------- ---------- user ---------- ---------- */

#[repr(align(4096))]
#[derive(Copy, Clone)]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

static USER_STACK: [UserStack; MAX_APP_NUM] = [UserStack {
    data: [0; USER_STACK_SIZE],
}; MAX_APP_NUM];

/* ---------- ---------- load ---------- ---------- */

/// 获取 app[id] 的 base
fn get_base_i(app_id: usize) -> usize {
    APP_BASE_ADDRESS + app_id * APP_SIZE_LIMIT
}

/// Get the total number of applications.
pub fn get_num_app() -> usize {
    extern "C" {
        fn _num_app();
    }
    unsafe { (_num_app as usize as *const usize).read_volatile() }
}

/// 结构可以参考 build.rs . 这里是初始化 .text 空间。
/// 然而, USER_STACK 和 KERNEL_STACK 在 lazy_static! 中初始化的
pub fn load_apps() {
    extern "C" {
        fn _num_app(); // 全局变量
    }
    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = get_num_app();

    // apps 的起始地址 vec
    let apps_start = unsafe {
        core::slice::from_raw_parts(
            num_app_ptr.add(1), /* 这里的 add(1) 表示: 跳过了一个数据类型, 这里是 usize */
            num_app + 1,
        )
    };

    // load apps
    for i in 0..num_app {
        let base_i = get_base_i(i); // app[i] 表示第 i 个应用的地址空间, base_i 就已经是地址了
        (base_i..base_i + APP_SIZE_LIMIT) // 初始化
            .for_each(|addr| unsafe { (addr as *mut u8).write_volatile(0) });

        // .text
        let srcs = unsafe {
            core::slice::from_raw_parts(
                apps_start[i] as *const u8,
                apps_start[i + 1] - apps_start[i],
            )
        };

        let dst = unsafe { core::slice::from_raw_parts_mut(base_i as *mut u8, srcs.len()) };
        dst.copy_from_slice(srcs);
    }
    // Memory fence about fetching the instruction memory
    // It is guaranteed that a subsequent instruction fetch must
    // observes all previous writes to the instruction memory.
    // Therefore, fence.i must be executed after we have loaded
    // the code of the next app into the instruction memory.
    // See also: riscv non-priv spec chapter 3, 'Zifencei' extension.
    unsafe {
        asm!("fence.i");
    }
}

/// get app info with entry and sp and save `TrapContext` in kernel stack
pub fn init_app_cx(app_id: usize) -> usize {
    // 返回的是 kernel_stack 的 sp
    KERNEL_STACK[app_id].push_context(
        // 创建一个 app_init_context
        TrapContext::app_init_context(get_base_i(app_id), USER_STACK[app_id].get_sp()),
    )
}
