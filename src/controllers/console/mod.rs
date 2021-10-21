mod screen;
pub use screen::*;

#[macro_use]
mod macros;
pub use macros::*;

use core::{
    fmt::{
        Write
    },
    default::Default
};

use crate::sys::KError;

pub trait ConsoleController : Write + Default {
    /// Return X,Y position
    fn get_xy(&self) -> (usize, usize);
    /// Set X,Y position
    fn set_xy(&mut self, x: usize, y: usize) -> Result<(), KError>;
    /// Get size in Columns, Rows
    fn get_size(&self) -> (usize, usize);
}