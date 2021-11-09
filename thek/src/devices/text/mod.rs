//! Text devices.

use super::{
    Id, Interrupt
};

use crate::{
    sys::KError
};

use ansi::AnsiColor;

pub mod arch;
pub mod ansi;

/// Text screen cursor shape.
pub enum CursorShape {
    FullBlock,
    HalfBlock,
    UnderLine,
    Default
}

/// Text screen cursor blinking.
pub enum CursorBlink {
    Fast,
    Slow,
    None,
    Default
}

/// Text device interface.
pub trait Text : Id + Interrupt {
    /// Put a character at X,Y position, not changing the color.
    fn put_char(&self, x: usize, y: usize, ch: char) -> Result<(), KError>;
    /// Put color at X,Y position, not changing the character.
    fn put_color(&self, x: usize, y: usize, text_color: AnsiColor, bg_color: AnsiColor) -> Result<(), KError>;
    /// Print one char with color at X,Y position.
    fn write(&self, x: usize, y: usize, text_color: AnsiColor, bg_color: AnsiColor, ch: char) -> Result<(), KError> {
        self.put_char(x, y, ch)?;
        self.put_color(x, y, text_color, bg_color)
    }
    /// Read char and color at X,Y position, return char, text color and background color in this order.
    fn read(&self, x: usize, y: usize) -> Result<(char, AnsiColor, AnsiColor), KError>;
    /// Set cursor X,Y position.
    fn set_position(&self, x: usize, y: usize) -> Result<(), KError>;
    /// Get cursor X,Y position.
    fn get_position(&self) -> Result<(usize, usize), KError>;
    /// Config cursor options.
    fn config_cursor(&self, enabled: bool, shape: CursorShape, blink: CursorBlink) -> Result<(), KError>;
    /// Get screen size in Columns,Rows.
    fn size(&self) -> Result<(usize, usize), KError>;
}
