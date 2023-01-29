use crate::vga_buffer::{Color::*, ColorCode};
use crate::{
    blackln, brownln, cyanln, greenln, lightredln, magentaln, println, redln, whiteln, yellowln,
};

#[test_case]
fn simple_print() {
    println!("test print");
}

#[test_case]
fn println_x500() {
    for _i in 0..500 {
        println!("test print");
    }
}

#[test_case]
fn color_print() {
    redln!("red test");
    yellowln!("red test");
    magentaln!("red test");
    greenln!("red test");
    cyanln!("red test");
    blackln!("red test");
    whiteln!("red test");
    lightredln!("red test");
    brownln!("red test");
}
