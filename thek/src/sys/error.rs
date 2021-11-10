/// Error type
#[derive(Copy, Clone, Debug)]
pub enum KError {
    /// Index out of bounds
    OutBounds,
    /// Segment stack is full
    FullSegStack,
    /// Not classified error
    Other
}

impl KError {
    pub fn msg(&self) -> &str {
        match self {
            KError::OutBounds => "Index out of bounds",
            KError::FullSegStack => "Segment stack is full",
            KError::Other => "Generic error",
        }
    }
}

impl core::default::Default for KError {
    fn default() -> Self {
        Self::Other
    }
}