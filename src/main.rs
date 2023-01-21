#![no_std] // Dont use the Rust standard library
#![no_main] // Dont use rust entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "tests"]

mod vga_buffer;
use crate::vga_buffer::{Color, ColorCode, WRITER};
use core::panic::PanicInfo;

// reminder of what mangling is: https://en.wikipedia.org/wiki/Name_mangling
#[no_mangle] // Dont mangle the function name
pub extern "C" fn _start() -> ! {
    init();
    println!(
        "omg imagine this works first try {}{}",
        "xd", "xd (it actually does wtf)"
    );
    #[cfg(test)]
    tests();
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests...", tests.len());

    for test in tests {
        test();
    }
}

//https://os.phil-opp.com/vga-text-mode/
// Panic function
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    WRITER
        .lock()
        .change_color(ColorCode::new(Color::Black, Color::LightRed));
    println!("Bruh moment! {}", _info);

    WRITER
        .lock()
        .change_color(ColorCode::new(Color::Black, Color::White));
    loop {}
}

fn init() {
    print!("Clarissa");
    WRITER
        .lock()
        .change_color(ColorCode::new(Color::Black, Color::DarkGray));
    println!("   // ver 0.1 - insane");
    WRITER
        .lock()
        .change_color(ColorCode::new(Color::Black, Color::White));
}
