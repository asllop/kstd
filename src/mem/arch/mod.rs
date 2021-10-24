//! Architectue dependant memory infrastructure.

// TODO: select arch by feature
pub mod x86_64;
pub use x86_64::*;