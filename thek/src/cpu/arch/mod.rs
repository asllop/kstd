//! Arch dependent CPU handling.
//! 
//! This module must provide, at least, the following public symbols:
//! 
//! - `fn init()`
//! - `fn start()`
//! - `fn halt()`
//! - `fn disable_ints()` 
//! - `fn enable_ints()`
//! - `fn check_ints() -> bool`
//! - `fn set_timer_handler(func: fn())`
//! - `const TIMER_FREQ_HZ: u64`

#[cfg(feature = "pc64")]
mod x86_64;
#[cfg(feature = "pc64")]
pub use self::x86_64::*;
