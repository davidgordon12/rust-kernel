#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

// entry point
#[no_mangle] // do not mangle the name of this function while compiling
pub extern "C" fn _start() -> !
{
    // this function is the entry point, 
    //since the linker needs an entry point
    loop {}
}

#[panic_handler] // this function will be called on panic
fn panic(_info: &PanicInfo) -> ! 
{
    loop {}
}
