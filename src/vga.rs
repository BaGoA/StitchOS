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
    fn new(foreground: Color, background: Color) -> Self {
        return Self {
            column_position: 0,
            color_code: ColorCode::new(foreground, background),
            buffer: unsafe {
                core::slice::from_raw_parts_mut(
                    0xb8000 as *mut ScreenChar,
                    BUFFER_HEIGHT * BUFFER_WIDTH,
                )
            },
        };
    }
}
