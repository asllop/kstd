//! Storage devices.

use super::{
    Id, Interrupt
};

use crate::sys::KError;

/// Storage device interface.
pub trait Storage : Id + Interrupt {
    /// Seek to `position` in sectors.
    /// * Return: actual position after seek.
    fn seek(&self, position: usize) -> Result<usize, KError>;
    /// Get current position in sectors.
    fn position(&self) -> Result<usize, KError>;
    /// Sector size in bytes.
    fn sector(&self) -> Result<usize, KError>;
    /// Read `size` sectors into `buffer`. Must be big enough to allocate size * sector_size bytes.
    /// * Return: actual bytes read.
    fn read(&self, size: usize, buffer: &mut u8) -> Result<usize, KError>;
    /// Write `size` sectors from `buffer`. Must be big enough to contain size * sector_size bytes.
    /// * Return: actual bytes written.
    fn write(&self, size: usize, buffer: &u8) -> Result<usize, KError>;
}