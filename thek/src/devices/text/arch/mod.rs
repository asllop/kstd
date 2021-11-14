//! Architecture dependent text devices.

#[cfg(feature = "pc64")]
mod pc;
#[cfg(feature = "pc64")]
pub use self::pc::*;
