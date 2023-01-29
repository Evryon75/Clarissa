#![no_std] // Dont use the Rust standard library
#![no_main] // Dont use rust entry points
#![feature(custom_test_frameworks)] // Lines 3, 4, 5: For testing purposes
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "run_tests"]

#[path = "buffer/color_macros.rs"]
mod color_macros;
#[path = "testing/serial.rs"]
mod serial;
#[path = "buffer/vga_buffer.rs"]
mod vga_buffer;
#[path = "testing/vga_print_tests.rs"]
#[cfg(test)]
mod vga_print_tests;

#[allow(unused_imports)]
use crate::color_macros::*;
#[allow(unused_imports)]
use crate::vga_buffer::{Color::*, ColorCode};
use core::fmt::Debug;
use core::panic::PanicInfo;
use x86_64::instructions::port::Port;

const VER: &str = "0.0.3";
const NAME: &str = "agony";

// reminder of what mangling is: https://en.wikipedia.org/wiki/Name_mangling
#[no_mangle] // Dont mangle the function name
pub extern "C" fn _start() -> ! {
    init();
    loop {}
}

// Panic function
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Modern registry for kernel panics 😌
    lightredln!("Bruh moment! {}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println_serial!("Failure: {}", _info);
    exit_qemu(ExitCode::Failure);
    loop {}
}

fn init() {
    redln!("Clarissa");
    darkgrayln!("        \\\\");
    lightgrayln!("          Ver {} - {}", VER, NAME);
    #[cfg(test)]
    run_tests();
}

pub trait Test {
    fn run(&self) -> ();
}
impl<T> Test for T
where
    T: Fn(),
{
    fn run(&self) -> () {
        print_serial!("{} >> ", core::any::type_name::<T>());
        self();
        println_serial!("Success");
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Test]) {
    // The dyn Fn() type allows us to pass functions or closures we can run
    println_serial!(
        "\n\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\nRunning {} tests...",
        tests.len()
    );
    for test in tests {
        test.run();
    }
    println_serial!("//////////////////////////////////////////////////////////////////////////\n");
    exit_qemu(ExitCode::Success);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ExitCode {
    Success = 0x10,
    Failure = 0x11,
}
#[allow(dead_code)]
fn exit_qemu(exit_code: ExitCode) {
    unsafe {
        // Make a port at 0xf4 on the io port bus and pass the exit code as the result
        Port::new(0xf4).write(exit_code as u32);
    }
}

// Plans: Make tetris IN the operating system insanity
