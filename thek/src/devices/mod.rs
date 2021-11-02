//! Abstraction types for direct hardware access.

pub mod plot;

use crate::sys::KMutex;

/// The trait that all devices must implement.
pub trait Device<'a> {
    /// Return the kernel mutex that holds the device.
    fn mutex() -> &'a KMutex<Self> where Self: Sized;
}