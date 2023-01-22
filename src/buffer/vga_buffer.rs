use core::fmt;
use core::fmt::Write;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;
#[allow(unused_imports)]
use crate::vga_buffer::Color::*;

#[allow(dead_code)]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(u8)] // Displays memory as u8
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct ColorCode(u8);
impl ColorCode {
    pub fn new(background: Color, foreground: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, Copy, Clone)]
struct VgaChar {
    character: u8,
    color: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

struct Buffer {
    chars: [[Volatile<VgaChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column: usize,
    color: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
    pub fn write_byte(&mut self, byte: u8) {
        if byte == b'\n' {
            self.new_line();
        } else {
            let row = BUFFER_HEIGHT - 1;
            let col = self.column;
            self.buffer.chars[row][col + 1].write(VgaChar {
                character: byte,
                color: self.color,
            });
            self.column += 1;
        }
    }
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.buffer.chars[row - 1][col].write(self.buffer.chars[row][col].read())
            }
        }
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[BUFFER_HEIGHT - 1][col].write(VgaChar {
                character: b' ',
                color: self.color,
            });
        }
        self.column = 0;
    }
    pub fn change_color(&mut self, color: ColorCode) {
        self.color = color
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column: 0,
        color: ColorCode::new(Color::Black, Color::White),
        // Make a mutable pointer to the vga buffer, dereference it, and borrow it as a mutable Buffer
        buffer: unsafe {&mut *(0xb8000 as *mut Buffer)}
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}