#![no_std] // Dont use the Rust standard library
#![no_main] // Dont use rust entry points

mod vga_buffer;

// reminder of what mangling is: https://en.wikipedia.org/wiki/Name_mangling
#[no_mangle] // Dont mangle the function name
pub extern "C" fn _start() -> ! {
    write("awesome");
    loop {}
}

// Panic function
use core::panic::PanicInfo;
use crate::vga_buffer::write;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
