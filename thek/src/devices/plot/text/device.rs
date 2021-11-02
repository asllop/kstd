use crate::{
    controllers::{
        console::{
            ansi::{
                AnsiColor
            }
        }
    },
    sys::KError
};

use super::super::super::Device;

/// Plot text device interface trait. All plot text devices must implement it.
pub trait PlotTextDevice<'a> : Device<'a> {

    /// Print one char with color at X,Y position
    /// 
    /// Must NOT update cursor, this should be done in the controller.
    fn print(&self, x: usize, y: usize, text_color: AnsiColor, bg_color: AnsiColor, ch: u8) -> Result<(), KError> {
        self.set_char(x, y, ch)?;
        self.set_color(x, y, text_color, bg_color)
    }

    /// Set character at X,Y position, not changing the color.
    fn set_char(&self, x: usize, y: usize, ch: u8) -> Result<(), KError>;

    /// Set color at X,Y position, not changing the character.
    fn set_color(&self, x: usize, y: usize, text_color: AnsiColor, bg_color: AnsiColor) -> Result<(), KError>;

    /// Read char and color at X,Y position, return char, text color and background color in this order
    fn read(&self, x: usize, y: usize) -> Result<(u8, AnsiColor, AnsiColor), KError>;

    /// Set cursor X,Y position
    fn set_cursor(&self, x: usize, y: usize) -> Result<(), KError>;

    /// Get cursor X,Y position
    fn get_cursor(&self) -> Result<(usize, usize), KError>;

    /// Enable cursor (set visible)
    fn enable_cursor(&self) -> Result<(), KError>;

    /// Disable cursor (set invisible)
    fn disable_cursor(&self) -> Result<(), KError>;

    /// Get console size in Columns,Rows
    fn get_size(&self) -> Result<(usize, usize), KError>;
}
