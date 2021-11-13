//! CPU handling.

pub mod arch;

/// Initialize CPU structures, registers, ints, etc.
pub fn init() {
    arch::init();
}
