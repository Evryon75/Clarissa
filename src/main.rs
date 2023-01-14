#![no_std] // Dont use the Rust standard library
#![no_main] // Dont use rust entry points

mod vga_buffer;
use crate::vga_buffer::{Color, ColorCode, WRITER};
use core::panic::PanicInfo;

// reminder of what mangling is: https://en.wikipedia.org/wiki/Name_mangling
#[no_mangle] // Dont mangle the function name
#[allow(unconditional_panic)] // TODO: This is just to test the panic function, REMOVE THIS!!!
pub extern "C" fn _start() -> ! {
    init();
    println!(
        "omg imagine this works first try {}{}",
        "xd", "xd (it actually does wtf)"
    );
    println!("{}", 4 / 0);

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
