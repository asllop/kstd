//! Console constrollers.

mod screen;
pub use screen::*;

pub mod ansi;

use core::{
    fmt::{
        Write
    },
    default::Default
};

use crate::sys::KError;

/// Console controller trait. All console controllers must implement it.
pub trait ConsoleController : Write + Default {
    /// Return X,Y position
    fn get_xy(&self) -> (usize, usize);
    /// Set X,Y position
    fn set_xy(&mut self, x: usize, y: usize) -> Result<(), KError>;
    /// Get size in Columns, Rows
    fn get_size(&self) -> (usize, usize);
}

use crate::{
    devices::{
        plot::{
            text::{
                ScreenTextDevice
            }
        }
    }
};

/// Default console controller
pub type DefaultConsoleController<'a> = ScreenConsoleController<'a, ScreenTextDevice>;