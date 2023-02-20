use crate::vga_buffer::Color::*;
use crate::vga_buffer::{ColorCode, WRITER};
use core::fmt;
use core::fmt::Write;
use x86_64::instructions::interrupts;
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
    interrupts::without_interrupts(|| {
        WRITER.lock().change_color(color);
        WRITER.lock().write_fmt(args).unwrap();
        WRITER.lock().change_color(ColorCode::new(Black, White));
    });
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
// SINGLE LINES
#[macro_export]
macro_rules! black {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, Black), "{}", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! blue {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, Blue), "{}", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! green {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, Green), "{}", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! cyan {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, Cyan), "{}", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! red {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, Red), "{}", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! magenta {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, Magenta), "{}", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! brown {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, Brown), "{}", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! lightgray {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, LightGray), "{}", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! darkgray {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, DarkGray), "{}", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! lightblue {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, LightBlue), "{}", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! lightgreen {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, LightGreen), "{}", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! lightcyan {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, LightCyan), "{}", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! lightred {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, LightRed), "{}", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! pink {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, Pink), "{}", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! yellow {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, Yellow), "{}", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! white {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_color!(ColorCode::new(Black, White), "{}", format_args!($($arg)*)));
}
