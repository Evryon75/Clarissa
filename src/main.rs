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

// reminder of what mangling is: https://en.wikipedia.org/wiki/Name_mangling
#[no_mangle] // Dont mangle the function name
pub extern "C" fn _start() -> ! {
    init();

    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    run_tests();
    loop {}
}

// Panic function
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // Modern registry for kernel panics 😌
    lightredln!("Bruh moment! {}", _info);
    loop {}
}

// Plans: Make tetris IN the operating system insanity
