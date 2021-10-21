use core::{
    marker::PhantomData
};

mod mutex;
pub use mutex::*;

mod error;
pub use error::*;

/// Empty enum
pub enum Empty {}

/// Alias of phantom data with Empty type
pub type Void = PhantomData<Empty>;