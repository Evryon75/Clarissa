#![no_std] // Dont use the Rust standard library
#![no_main] // Dont use rust entry points
#![feature(custom_test_frameworks)] // Lines 3, 4, 5: For testing purposes
#![test_runner(clarissa::test_runner)]
#![reexport_test_harness_main = "run_tests"]

#[allow(unused_imports)]
use clarissa::color_macros::*;
use clarissa::vga_buffer::Color::*;
use clarissa::vga_buffer::*;
use clarissa::*;

const VER: &str = "0.1.4"; // y.x.z = Section Z from chapter X, if Y is 0 the guide isn't finished, if its 1 or above it is
const NAME: &str = "testing more like";

// reminder of what mangling is: https://en.wikipedia.org/wiki/Name_mangling
#[no_mangle] // Dont mangle the function name
pub extern "C" fn _start() -> ! {
    init();
    loop {}
}

// Panic function
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // Modern registry for kernel panics 😌
    lightredln!("Bruh moment! {}", _info);
    loop {}
}

#[test_case]
fn function27() {
    return ();
}
fn init() {
    redln!("Clarissa");
    darkgrayln!("        \\\\");
    lightgrayln!("          Ver {} - {}", VER, NAME);

    /*
    ERROR
    #[test_case]
    fn function27() {
        return ();
    }
    */

    #[cfg(test)]
    println_serial!("\n          [main.rs]");
    #[cfg(test)]
    run_tests();
}

// Plans: Make tetris IN the operating system insanity
