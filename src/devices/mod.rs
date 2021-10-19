pub mod console;
pub use console::*;

use core::fmt::Error;

/// Input Flow trait. For writing data to a device.
pub trait InputFlow<T> {
    type Command;

    fn write_cmd(&self, cmd: Self::Command, data: T) -> Result<(), Error>;
}

/// Output Flow trait. For reading data from a device.
pub trait OutputFlow<T> {
    type Command;
    
    fn read_cmd(&self, cmd: Self::Command) -> Option<T>;
}