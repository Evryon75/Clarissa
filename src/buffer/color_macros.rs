use core::fmt;
use core::fmt::Write;
use crate::vga_buffer::Color::*;
use crate::vga_buffer::{ColorCode, WRITER};
#[macro_export]
macro_rules! print_color {
    ($color: expr, $($arg:tt)*) => ($crate::color_macros::_print_color($color, format_args!($($arg)*)));
}
#[macro_export]
macro_rules! println_color {
    () => ($crate::print!("\n"));
    ($color: expr, $($arg:tt)*) => ($crate::print_color!($color, "{}\n", format_args!($($arg)*)));
}
#[doc(hidden)]
pub fn _print_color(color: ColorCode, args: fmt::Arguments) {
    WRITER.lock().change_color(color);
    WRITER.lock().write_fmt(args).unwrap();
    WRITER.lock().change_color(ColorCode::new(Black, White));
}

#[macro_export]
macro_rules! blackln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, Black), "{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! blueln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, Blue), "{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! greenln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, Green), "{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! cyanln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, Cyan), "{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! redln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, Red), "{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! magentaln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, Magenta), "{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! brownln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, Brown), "{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! lightgrayln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, LightGray), "{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! darkgrayln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, DarkGray), "{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! lightblueln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, LightBlue), "{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! lightgreenln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, LightGreen), "{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! lightcyanln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, LightCyan), "{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! lightredln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, LightRed), "{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! pinkln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, Pink), "{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! yellowln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, Yellow), "{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! whiteln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, White), "{}\n", format_args!($($arg)*)));
}
