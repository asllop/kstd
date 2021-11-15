//! CPU handling.

pub mod arch;

pub use arch::{
    init, start, halt
};
