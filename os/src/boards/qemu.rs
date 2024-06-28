//! Constants used in rCore for qemu

pub const CLOCK_FREQ: usize = 12500000;

pub const MEMORY_END: usize = 0x8800_0000;

/// 1. MMIO 区域的起始地址 = 0x0010_0000
/// 2. MMIO 大小是 0x2000 ( 8192 ) 字节
pub const MMIO: &[(usize, usize)] = &[
    (0x0010_0000, 0x00_2000), // VIRT_TEST/RTC  in virt machine
];
