#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "run_tests"]

use core::panic::PanicInfo;
use x86_64::instructions::port::Port;

use crate::vga_buffer::Color::*;
use crate::vga_buffer::ColorCode;

#[path = "buffer/color_macros.rs"]
pub mod color_macros;
pub mod interrupts;
#[path = "buffer/serial.rs"]
pub mod serial;
#[path = "buffer/vga_buffer.rs"]
pub mod vga_buffer;

#[no_mangle]
#[cfg(test)]
pub extern "C" fn _start() -> ! {
    init();
    println_serial!("\n          [lib.rs]");
    run_tests();
    loop {}
}

const VER: &str = "0.2.1"; // y.x.z = Section Z from chapter X, if Y is 0 the guide isn't finished, if its 1 or above it is
const NAME: &str = "dude i just had the flu, couldnt code for a week (agony)";

pub fn init() {
    redln!("Clarissa");
    darkgrayln!("        \\\\");
    lightgrayln!("          Ver {} - {}", VER, NAME);

    interrupts::initialize_idt();
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

pub fn test_runner(tests: &[&dyn Test]) {
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

pub fn test_panic(_info: &PanicInfo) {
    println_serial!("Failure: {}", _info);
    exit_qemu(ExitCode::Failure);
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    test_panic(_info);
    loop {}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ExitCode {
    Success = 0x10, // 33
    Failure = 0x11, // 34
}

#[allow(dead_code)]
pub fn exit_qemu(exit_code: ExitCode) {
    unsafe {
        // Make a port at 0xf4 on the io port bus and pass the exit code as the result
        Port::new(0xf4).write(exit_code as u32);
    }
}
