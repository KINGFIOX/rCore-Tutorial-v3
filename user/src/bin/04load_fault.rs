#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use core::ptr::read_volatile;

#[no_mangle]
fn main() -> i32 {
    println!("\nload_fault APP running...\n");
    println!("Into Test load_fault, we will insert an invalid load operation...");
    println!("Kernel should kill this application!");
    unsafe {
        let _i: u8 = read_volatile(core::ptr::NonNull::dangling().as_ptr());
    }
    0
}
