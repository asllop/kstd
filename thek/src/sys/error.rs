/// Error type
#[derive(Copy, Clone)]
pub enum KError {
    /// Index out of bounds
    OutBounds,
    /// Wrong command
    WrongCmd,
    /// Not classified error
    Other
}

impl KError {
    pub fn msg(&self) -> &str {
        //TODO: convert code into a message
        ""
    }
}

impl core::default::Default for KError {
    fn default() -> Self {
        Self::Other
    }
}