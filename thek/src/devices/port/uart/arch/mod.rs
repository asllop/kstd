//! UART port devices, arch dependent parts.

//TODO: use feature to select arch
mod pc;
pub use pc::*; 