#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(clarissa::test_runner)]
#![reexport_test_harness_main = "run_tests"]

#[allow(unused_imports)]
use clarissa::color_macros::*;
use clarissa::vga_buffer::Color::*;
use clarissa::vga_buffer::*;
use clarissa::*;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println_serial!("\n          [basic_boot.rs]");
    run_tests();

    loop {}
}

#[test_case]
fn printing() {
    println!("Test print");
    lightredln!("Test color print");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic(info);
    loop {}
}
