use crate::{
    sys::KError as Error
};

/// Input Flow trait. For writing data to a device.
/// 
/// The generic types are:
/// * `D` : Data to write. 
/// * `C` : Command to run.
/// * `R` : Result to return.
pub trait InputFlow<D, C, R> {
    fn write_cmd(&self, cmd: C, data: D) -> Result<R, Error>;
}

/// Output Flow trait. For reading data from a device.
/// 
/// The generic types are:
/// * `D` : Data to return. 
/// * `C` : Command to run.
pub trait OutputFlow<D, C> {
    fn read_cmd(&self, cmd: C) -> Result<D, Error>;
}