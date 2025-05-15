#![allow(dead_code)]

/// VGA color palette in byte representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    Write = 15,
}

/// VGA color code used to describe foreground and background color together
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    /// Create a ColorCode from foreground and background color
    fn new(foreground: Color, background: Color) -> Self {
        return Self((background as u8) << 4 | (foreground as u8));
    }
}

/// Representation of character displayed on screen via VGA text-mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const BUFFER_ADDRESS: usize = 0xb8000;

/// Compute the index of the memory block from row and column indexes
fn index_memory_block(row_index: usize, column_index: usize) -> usize {
    return row_index * BUFFER_WIDTH + column_index;
}

/// Writer to write in VGA buffer
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut [ScreenChar],
}

impl Writer {
    /// Create a VGA buffer writer from foreground and background color
    pub fn new(foreground: Color, background: Color) -> Self {
        return Self {
            column_position: 0,
            color_code: ColorCode::new(foreground, background),
            buffer: unsafe {
                core::slice::from_raw_parts_mut(
                    core::ptr::with_exposed_provenance_mut::<ScreenChar>(BUFFER_ADDRESS),
                    BUFFER_HEIGHT * BUFFER_WIDTH,
                )
            },
        };
    }

    /// Write a byte in VGA buffer with foreground and background color given
    /// at the construction of the Writer
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            _ => {
                if self.column_position >= BUFFER_WIDTH - 1 {
                    // the current line is full
                    self.new_line();
                }

                // The writer write always to the last line
                let index_in_buffer: usize =
                    index_memory_block(BUFFER_HEIGHT - 1, self.column_position);

                self.buffer[index_in_buffer] = ScreenChar {
                    ascii_char: byte,
                    color_code: self.color_code,
                };

                self.column_position += 1;
            }
        }
    }

    /// Write a string in VGA buffer with foreground and background color given
    /// at the construction of the Writer
    pub fn write_string(&mut self, string: &str) {
        string.bytes().for_each(|byte: u8| {
            match byte {
                0x20..=0x7e | b'\n' => {
                    // the character is a printable by VGA
                    self.write_byte(byte)
                }
                _ => self.write_byte(0xfe), // for unprintable character we write the white square character
            }
        });
    }

    fn new_line(&mut self) {
        // TODO
    }
}
