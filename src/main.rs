#![no_std] // Dont use the Rust standard library
#![no_main] // Dont use rust entry points
#![feature(custom_test_frameworks)] // Lines 3, 4, 5: For testing purposes
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "run_tests"]

#[path = "buffer/vga_buffer.rs"]
mod vga_buffer;
#[path = "buffer/color_macros.rs"]
mod color_macros;

#[allow(unused_imports)]
use crate::vga_buffer::{Color::*, ColorCode};
#[allow(unused_imports)]
use crate::color_macros::*;
use core::panic::PanicInfo;

const VER: &str = "0.0.2";
const NAME: &str = "making stuff i think ehh gotta go shower and then watch blade runner";

// reminder of what mangling is: https://en.wikipedia.org/wiki/Name_mangling
#[no_mangle] // Dont mangle the function name
pub extern "C" fn _start() -> ! {
    init();
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()] /* The dyn Fn() type allows us to pass functions or closures we can run */) {
    println!("Running {} tests...", tests.len());

    for test in tests {
        test();
    }
}

// Panic function
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Modern naming conventions to kernel panics 😌
    lightredln!("Bruh moment! {}", _info);
    loop {}
}

fn init() {
    redln!("Clarissa");
    darkgrayln!("        \\\\");
    lightgrayln!("          Ver {} - {}", VER, NAME);

    #[cfg(test)]
    run_tests();
}
