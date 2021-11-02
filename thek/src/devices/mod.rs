//! Abstraction types for direct hardware access.

pub mod plot;

use crate::sys::KLock;

/// Trait that all devices must implement.
pub trait Device<'a> {
    /// Acquire lock on device
    fn lock() ->  KLock<'a, Self> where Self: Sized;
    /// Reset lock. For emergency cases only.
    fn reset_lock();
}