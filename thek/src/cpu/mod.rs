//! CPU handling.

pub mod arch;

pub mod time;

pub use arch::{
    init, start, halt
};
