use core::{
    marker::PhantomData
};

/// Empty enum
pub enum Empty {}

/// Alias of phantom data with Empty type
pub type Void = PhantomData<Empty>;
