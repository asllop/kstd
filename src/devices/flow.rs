use crate::{
    sys::KError as Error
};

/// Input Flow trait. For writing data to a device.
pub trait InputFlow<T> {
    type Command;
    type CmdResult;

    fn write_cmd(&self, cmd: Self::Command, data: T) -> Result<Self::CmdResult, Error>;
}

/// Output Flow trait. For reading data from a device.
pub trait OutputFlow<T> {
    type Command;
    
    fn read_cmd(&self, cmd: Self::Command) -> Option<T>;
}