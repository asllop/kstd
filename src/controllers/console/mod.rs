mod screen;
pub use screen::*;

#[macro_use]
mod macros;
pub use macros::*;

use core::{
    fmt::{
        Write
    }
};

use crate::sys::KError as Error;

pub trait ConsoleController : Write {
    fn x(&self) -> usize;
    fn y(&self) -> usize;
    fn set_x(&self, x: usize) -> Result<(), Error>;
    fn set_y(&self, y: usize) -> Result<(), Error>;
    fn cols(&self) -> usize;
    fn rows(&self) -> usize;
}