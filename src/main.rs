#![no_std] // Dont use the Rust standard library
#![no_main] // Dont use rust entry points

mod vga_buffer;

// reminder of what mangling is: https://en.wikipedia.org/wiki/Name_mangling
#[no_mangle] // Dont mangle the function name
pub extern "C" fn _start() -> ! {
    write("omg hi 123456789 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41");
    loop {}
}
//https://os.phil-opp.com/vga-text-mode/
// Panic function
use core::panic::PanicInfo;
use crate::vga_buffer::write;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}