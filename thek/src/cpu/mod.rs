//! CPU handling.

pub mod arch;

pub mod time;

pub use arch::{
    start_arch as start_cpu, halt, disable_ints, enable_ints, check_ints
};

/// Initialize ints, cpu structures, timers, etc.
pub fn init_cpu() {
    arch::init_arch();
    time::init_time();
}
