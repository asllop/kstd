//! Abstraction types for direct hardware access.

pub mod plot;

use crate::sys::KMutex;

/// Trait that all devices must implement.
pub trait Device<'a> {
    /// Return the k mutex that holds the driver.
    fn mutex() -> &'a KMutex<Self> where Self: Sized;
}