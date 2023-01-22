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

const VER: &str = "0.1.0";
const NAME: &str = "Insanity";

// reminder of what mangling is: https://en.wikipedia.org/wiki/Name_mangling
#[no_mangle] // Dont mangle the function name
pub extern "C" fn _start() -> ! {
    init();
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests...", tests.len());

    for test in tests {
        test();
    }
}

// https://os.phil-opp.com/vga-text-mode/
// Panic function
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
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
