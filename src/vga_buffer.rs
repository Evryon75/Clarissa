use core::ops::Shl;
use crate::vga_buffer::Color::{Black, Red, Yellow};

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

struct ColorCode(u8);
impl ColorCode {
    fn new(background: Color, foreground: Color) -> ColorCode {
        ColorCode(background << 4 | foreground)
    }
}

struct VgaChar {
    character: u8,
    color: ColorCode
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

struct Buffer {
    chars: [[VgaChar; BUFFER_HEIGHT]; BUFFER_WIDTH]
}

pub struct Writer {
    column: usize,
    color: ColorCode,
    buffer: &'static mut Buffer
}
impl Writer {
    pub fn write(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe)
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        if byte == b'\n' {
            self.new_line();
        } else {
            if self.column > BUFFER_WIDTH {
                self.new_line();
            }
            let row = BUFFER_HEIGHT - 1;
            let col = self.column;
            self.buffer.chars[row][col] = VgaChar {
                character: byte,
                color: self.color,
            };
            self.column += 1;
        }
    }
    fn new_line(&mut self) {
        // TODO: Add new line
    }
}

pub fn write(s: &str) {
    let mut writer = Writer {
        column: 0,
        color: ColorCode::new(Red, Black),
        buffer: unsafe {&mut *(0xb8000 as *mut Buffer)}
    };
    writer.write(s)
}