//! Architectue dependant memory infrastructure.

#[cfg(feature = "pc64")]
mod x86_64;
#[cfg(feature = "pc64")]
pub use self::x86_64::*;
