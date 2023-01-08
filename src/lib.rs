#![feature(lang_items)]
#![no_std]

use core::panic::PanicInfo;

extern crate rlibc;

#[no_mangle]
pub extern fn kernel_main()
{
    // ATTENTION: we have a very small stack and no guard page

    let lysia = b"LysiaOS";
    let color_byte = 0x2f; // white foreground, green background

    let mut lysia_colored = [color_byte; 14];
    for (i, char_byte) in lysia.into_iter().enumerate() {
        lysia_colored[i*2] = *char_byte;
    }

    // write `LysiaOS` to the centre of the VGA text buffer
    let buffer_ptr = (0xb8000 + 1988) as *mut _;
    unsafe { *buffer_ptr = lysia_colored };

    loop{}
}

#[panic_handler]
pub extern fn panic_handler(panic_info: &PanicInfo) -> !
{
    loop {}
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}