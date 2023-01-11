#![feature(lang_items)]
#![feature(ptr_internals)]
#![no_std]

use core::panic::PanicInfo;

extern crate rlibc;
extern crate volatile;
extern crate multiboot2;

#[macro_use]
mod vga_buffer;
mod memory;

#[no_mangle]
pub extern fn kernel_main(multiboot_information_address: usize)
{
    use memory::FrameAllocator;

    vga_buffer::clear_screen();

    unsafe { print!("Lysia OS") };
    unsafe { print!("Version {}.{} ", 0, 1.1) };

    // The load function is unsafe because it relies on a valid address
    let boot_info = unsafe 
    { 
        multiboot2::load(multiboot_information_address) 
    };

    let memory_map_tag = boot_info
        .memory_map_tag()
        .expect("Memory map tag required");

    unsafe { print!("Memory areas:") };

    for area in memory_map_tag.memory_areas()
    {
        unsafe { print!("    start: 0x{:x}, length: 0x{:x}", area.base_addr, area.length) };
    } 

    unsafe { print!("End of Memory areas ") };

    let elf_sections_tag = boot_info.elf_sections_tag()
    .expect("Elf-sections tag required");

    unsafe { print!("Kernel sections: ") };

    for section in elf_sections_tag.sections()
    {
        unsafe { print!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
            section.addr, section.size, section.flags) };
    }

    unsafe { print!("End of Kernel sections") };

    let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
    .min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size)
    .max().unwrap();

    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    unsafe { print!("Kernel start: 0x{:x}, Kernel end: 0x{:x}", kernel_start, kernel_end) };
    unsafe { print!("Multiboot start: 0x{:x}, Multiboot end: 0x{:x}", multiboot_start, multiboot_end) };

    let mut frame_allocator = memory::AreaFrameAllocator::new(
        kernel_start as usize, kernel_end as usize, multiboot_start,
        multiboot_end, memory_map_tag.memory_areas());

    unsafe { print!("{:?}", frame_allocator.allocate_frame()) };

    for i in 0..
    {
        if let None = frame_allocator.allocate_frame()
        {
            unsafe { print!("Allocated {} frames", i)}
            break;
        }
    }

    loop{}
}

#[panic_handler]
#[allow(dead_code)]
pub extern fn panic_handler(panic_info: &PanicInfo) -> !
{
    unsafe { print!("System panic: {}. Please abort", panic_info) };
    loop {}
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}