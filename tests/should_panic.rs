#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "run_tests"]

#[allow(unused_imports)]
use clarissa::color_macros::*;
use clarissa::vga_buffer::Color::*;
use clarissa::vga_buffer::*;
use clarissa::*;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println_serial!("\n          [should_panic.rs]");
    run_tests();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println_serial!("Success");
    println_serial!("//////////////////////////////////////////////////////////////////////////\n");
    exit_qemu(ExitCode::Success);
    loop {}
}

pub trait PanicTest {
    fn run(&self) -> ();
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    // The dyn Fn() type allows us to pass functions or closures we can run
    println_serial!(
        "\n\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\nRunning {} tests...",
        tests.len()
    );
    for test in tests {
        test();
        println_serial!("Failure: The test did not panic");
    }
    println_serial!("//////////////////////////////////////////////////////////////////////////\n");
    exit_qemu(ExitCode::Failure);
}

#[test_case]
fn panic_test() {
    print_serial!("should_panic::panic_test >> ");
    assert_eq!(0, 1);
}
