//! Plot text devices.

//TODO: put each arch dependant ConsoleDevice implementation into a different module and compile conditionally
//TODO: use cargo features
pub mod arch_pc;

mod device;
pub use device::*;