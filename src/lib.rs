#![feature(lang_items)]
#![feature(ptr_internals)]
#![no_std]

use core::panic::PanicInfo;

extern crate rlibc;
extern crate volatile;

mod vga_buffer;

#[no_mangle]
pub extern fn kernel_main()
{
    vga_buffer::print_bytes("LysiaOS\n");
    vga_buffer::print_bytes("Welcome to version 0.1.1");

    loop{}
}

#[panic_handler]
pub extern fn panic_handler(panic_info: &PanicInfo) -> !
{
    loop {}
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}