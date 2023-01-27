#![no_std] // Dont use the Rust standard library
#![no_main] // Dont use rust entry points
#![feature(custom_test_frameworks)] // Lines 3, 4, 5: For testing purposes
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "run_tests"]

#[path = "buffer/color_macros.rs"]
mod color_macros;
#[path = "buffer/vga_buffer.rs"]
mod vga_buffer;

#[allow(unused_imports)]
use crate::color_macros::*;
#[allow(unused_imports)]
use crate::vga_buffer::{Color::*, ColorCode};
use core::panic::PanicInfo;
use x86_64::instructions::port::{Port};

const VER: &str = "0.0.2";
const NAME: &str = "s";

// reminder of what mangling is: https://en.wikipedia.org/wiki/Name_mangling
#[no_mangle] // Dont mangle the function name
pub extern "C" fn _start() -> ! {
    init();
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    // The dyn Fn() type allows us to pass functions or closures we can run
    println!("Running {} tests...", tests.len());

    for test in tests {
        test();
    }
    exit_qemu(ExitCode::Success);
}

// Panic function
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Modern registry for kernel panics 😌
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ExitCode {
    Success = 0x10,
    Failure = 0x11,
}
fn exit_qemu(exit_code: ExitCode) {
    unsafe {
        // Make a port at 0xf4 on the io port bus and pass the exit code as the result
        Port::new(0xf4).write(exit_code as u32);
    }
}

// Plans: Make tetris IN the operating system insanity
