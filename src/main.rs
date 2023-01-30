#![no_std] // Dont use the Rust standard library
#![no_main] // Dont use rust entry points

// HELLO has the type of bytes representing the ASCII code of the character
static HELLO: &[u8] = b"THIS IS INSANE!";

// reminder of what mangling is: https://en.wikipedia.org/wiki/Name_mangling
#[no_mangle] // Dont mangle the function name
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    // enumerate() returns couples consisting of the byte representing the character and its index in the array
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

// Panic function
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}