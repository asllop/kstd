//! CPU handling.

pub mod arch;

pub mod time;

pub use arch::{
    start, halt, disable_ints, enable_ints, check_ints
};

/// Initialize ints, cpu structures, timers, etc.
pub fn init() {
    arch::init();
    time::init_time();
}
