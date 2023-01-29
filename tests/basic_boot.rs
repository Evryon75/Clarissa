#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(clarissa::test_runner)]
#![reexport_test_harness_main = "run_tests"]

use clarissa::println_serial;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println_serial!("\n          [basic_boot.rs]");
    run_tests();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    clarissa::test_panic(info);
    loop {}
}
