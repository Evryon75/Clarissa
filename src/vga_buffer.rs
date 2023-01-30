#[allow(dead_code)]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(u8)]
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