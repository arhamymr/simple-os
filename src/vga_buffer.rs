// EXPERIMENTA CODE

// const VGA_WIDTH: usize = 80;
// const VGA_HEIGHT: usize = 25;
// const VGA_BUFFER: *mut u16 = 0xB8000 as *mut u16;

// pub fn write_char(x: usize, y: usize, c: u8) {
//     let offset = y * VGA_WIDTH + x;
//     unsafe {
//         *VGA_BUFFER.offset(offset as isize) = 0xb000 | c as u16;
//     }
// }

// pub fn clear_screen() {
//     for y in 0..VGA_HEIGHT {
//         for x in 0..VGA_WIDTH {
//             write_char(x, y, b' ');
//         }
//     }
// }

// pub fn print_string(s: &str) {
//     let mut x = 0;
//     let mut y = 0;

//     for &byte in s.as_bytes() {
//         match byte {
//             b'\n' => {
//                 x = 0;
//                 y += 1;
//             }

//             byte => {
//                 if x >= VGA_WIDTH {
//                     x = 0;
//                     y += 1;
//                 }

//                 if x >= VGA_HEIGHT {
//                     clear_screen();
//                     y = 0;
//                 }

//                 write_char(x, y, byte);
//                 x += 1
//             }
//         }
//     }
// }

//  TUTORIAL CODE
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Copy, Eq)]
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

#[derive(Debug, Clone, PartialEq, Copy, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line()
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;

                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };

                self.column_position += 1;
            }
        }
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col] = blank;
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col];
                self.buffer.chars[row - 1][col] = character;
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

pub fn write_something() {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("World !");
}
