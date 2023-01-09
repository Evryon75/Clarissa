#![no_std] // Dont use the Rust standard library
#![no_main] // Dont use rust entry points

// HELLO has the type of bytes representing the ASCII code of the character
static HELLO: &[u8] = b"THIS IS INSANE!";

// reminder of what mangling is: https://en.wikipedia.org/wiki/Name_mangling
#[no_mangle] // Dont mangle the function name
pub extern "C" fn _start() -> ! {
    write("hola");
    loop {}
}

// Panic function
use core::panic::PanicInfo;
use crate::vga_buffer::write;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}