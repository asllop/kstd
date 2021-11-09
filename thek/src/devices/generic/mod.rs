//! Generic devices.

use super::{
    Id, Interrupt
};

use crate::sys::KError;

/// Generic device interface.
pub trait Generic : Id + Interrupt {
    /// Read `size` bytes into `buffer`. Must be big enough to allocate size bytes.
    /// * Return: actual bytes read.
    fn read(&self, size: usize, buffer: &mut u8) -> Result<usize, KError>;
    /// Write `size` bytes from `buffer`. Must be big enough to contain size bytes.
    /// * Return: actual bytes written.
    fn write(&self, size: usize, buffer: &u8) -> Result<usize, KError>;
    /// Send `command` with optional `data`.
    /// * Return: command result.
    fn cmd(&self, command: usize, data: Option<&u8>) -> Result<Option<&u8>, KError>;
}
