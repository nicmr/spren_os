#![feature(panic_implementation)]
#![feature(use_extern_macros)]
#![no_std]
#![no_main]

extern crate volatile; //used in vga_buffer.rs

use core::panic::PanicInfo;

mod vga_buffer;

static HELLO: &[u8] = b"Hello World!";

#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop{}
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    //entry point, linker looks for _start function by default

    vga_buffer::print_boot_msg();
    loop {}
}


