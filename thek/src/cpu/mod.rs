//! CPU handling.

pub mod arch;

/// Initialize interrupts.
pub fn init_ints() {
    arch::init_ints();
}
