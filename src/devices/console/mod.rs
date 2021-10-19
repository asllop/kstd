mod ansi;
pub use ansi::*;

mod device;
pub use device::*;

//TODO: put each arch dependant ConsoleDevice implementation into a different module and compile conditionally
mod arch_pc;
pub use arch_pc::*;