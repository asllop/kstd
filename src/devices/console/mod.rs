//! Console device.

pub mod ansi;

mod device;
pub use device::*;

//TODO: put each arch dependant ConsoleDevice implementation into a different module and compile conditionally
//TODO: use cargo features
pub mod arch_pc;