#![feature(lang_items)]
#![no_std]

use core::panic::PanicInfo;

#[no_mangle]
pub extern fn kernel_main()
{
}

#[panic_handler]
pub extern fn panic_handler(panic_info: &PanicInfo) -> !
{
    loop {}
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}