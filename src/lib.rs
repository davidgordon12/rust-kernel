#![feature(lang_items)]
#![feature(ptr_internals)]
#![no_std]

use core::panic::PanicInfo;

extern crate rlibc;
extern crate volatile;
extern crate multiboot2;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern fn kernel_main(multiboot_information_address: usize)
{
    vga_buffer::clear_screen();

    unsafe { print!("Lysia OS\n") };
    unsafe { print!("Version {}.{} \n", 0, 1.1) };

    // The load function is unsafe because it relies on a valid address
    let boot_info = unsafe 
    { 
        multiboot2::load(multiboot_information_address) 
    };

    let memory_map_tag = boot_info
        .memory_map_tag()
        .expect("Memory map tag required");

    unsafe { print!("Memory areas: \n") };
    for area in memory_map_tag.memory_areas()
    {
        unsafe { print!("start: 0x{:x}, length: 0x{:x}", area.base_addr, area.length) };
    } 

    loop{}
}

#[panic_handler]
#[allow(dead_code)]
pub extern fn panic_handler(panic_info: &PanicInfo) -> !
{
    vga_buffer::print_line("System has panicked. Please abort");
    loop {}
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}