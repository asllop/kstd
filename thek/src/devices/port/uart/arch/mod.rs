//! UART port devices, arch dependent parts.

#[cfg(feature = "pc64")]
mod pc;
#[cfg(feature = "pc64")]
pub use pc::*;
