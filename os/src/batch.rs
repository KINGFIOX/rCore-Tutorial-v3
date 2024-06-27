//! batch subsystem

use crate::sbi::shutdown;
use crate::sync::UPSafeCell;
use crate::trap::TrapContext;
use core::arch::asm;
use lazy_static::*;

const MAX_APP_NUM: usize = 16;

// 其实是: 所有的 app 都是头一个其实地址
const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;

/* ---------- ---------- kernel stack ---------- ---------- */

const KERNEL_STACK_SIZE: usize = 4096 * 2;

#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

static KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};

impl KernelStack {
    fn get_sp(&self) -> usize {
        // 栈顶
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
    pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
        // 把 cx_ptr 指向了新的栈顶，然后复制一层 frame , 然后返回 kernel 的栈顶
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *cx_ptr = cx; // 相当于是深拷贝，一大块内存倒了一下
        }
        unsafe { cx_ptr.as_mut().unwrap() }
    }
}

/* ---------- ---------- user stack ---------- ---------- */

const USER_STACK_SIZE: usize = 4096 * 2;

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

static USER_STACK: UserStack = UserStack {
    data: [0; USER_STACK_SIZE],
};

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

/* ---------- ---------- app manager ---------- ---------- */

struct AppManager {
    /// AppManager 中. number of app
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
}

impl AppManager {
    pub fn print_app_info(&self) {
        println!("[kernel] num_app = {}", self.num_app);
        for i in 0..self.num_app {
            println!(
                "[kernel] app_{} [{:#x}, {:#x})",
                i,
                self.app_start[i],
                self.app_start[i + 1]
            );
        }
    }

    unsafe fn load_app(&self, app_id: usize) {
        if app_id >= self.num_app {
            println!("All applications completed!");
            shutdown(false);
        }
        println!("[kernel] Loading app_{}", app_id);
        // clear app area
        // text -> havard
        core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT) /* from_raw_parts_mut 返回 &'a mut [T] */
        .fill(0); /* slice fill(0) */
        // app 的 src 的 起始地址
        // app_src 指向的是 app image 的起始地址
        let app_src = core::slice::from_raw_parts(
            self.app_start[app_id] as *const u8,
            self.app_start[app_id + 1] - self.app_start[app_id],
        );
        // 复制到 APP_BASE_ADDRESS, 这个地方是被操作系统认的
        let app_dst = core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());
        app_dst.copy_from_slice(app_src);
        // Memory fence about fetching the instruction memory
        // It is guaranteed that a subsequent instruction fetch must
        // observes all previous writes to the instruction memory.
        // Therefore, fence.i must be executed after we have loaded
        // the code of the next app into the instruction memory.
        // See also: riscv non-priv spec chapter 3, 'Zifencei' extension.
        asm!("fence.i");
    }

    pub fn get_current_app(&self) -> usize {
        self.current_app
    }

    pub fn move_to_next_app(&mut self) {
        self.current_app += 1;
    }
}

/* ---------- ---------- 下一个 ---------- ---------- */

/// run next app
pub fn run_next_app() -> ! {
    let mut app_manager = APP_MANAGER.exclusive_access();
    let current_app = app_manager.get_current_app();
    unsafe {
        app_manager.load_app(current_app);
    }
    app_manager.move_to_next_app();
    // refcell drop
    drop(app_manager);
    // before this we have to drop local variables related to resources manually
    // and release the resources

    extern "C" {
        fn __restore(cx_addr: usize);
    }
    unsafe {
        // 相当于是: 准备好了 context, 然后恢复到 context
        __restore(
            //
            KERNEL_STACK.push_context(
                // 上下文要有 1. app 的地址; 2. app 的栈地址
                TrapContext::app_init_context(APP_BASE_ADDRESS, USER_STACK.get_sp()),
            ) as *const _ as usize,
        );
    }
    panic!("Unreachable in batch::run_current_app!");
}

/* ---------- ---------- init ---------- ---------- */

lazy_static! {
    static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {
        UPSafeCell::new({
            extern "C" {
                fn _num_app();
            }
            let num_app_ptr = _num_app as usize as *const usize;
            let num_app = num_app_ptr.read_volatile();  // 读取 num_app
            let mut app_start: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
            let app_start_raw: &[usize] =
                core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1);
            app_start[..=num_app].copy_from_slice(app_start_raw);
            AppManager {            // 构造对象
                num_app,
                current_app: 0,
                app_start,
            }
        })
    };
}

/// init batch subsystem
pub fn init() {
    print_app_info();
}

/// print apps info
pub fn print_app_info() {
    APP_MANAGER.exclusive_access().print_app_info();
}
