//! Arch dependent CPU handling.
//! 
//! This module must provide, at least, the following public symbols:
//! 
//! `fn init()`
//! `fn start()`
//! `fn halt()`
//! `fn set_timer_handler(func: fn(f64))`

#[cfg(feature = "pc64")]
mod x86_64;
#[cfg(feature = "pc64")]
pub use self::x86_64::*;
