/// Error type
#[derive(Copy, Clone)]
pub enum Error {
    /// Buffer out of bounds
    BufOutBounds,
    /// Wrong command
    WrongCmd,
    /// Not classified error
    Other
}

impl Error {
    pub fn msg(&self) -> &str {
        //TODO: convert code into a message
        ""
    }
}

impl core::default::Default for Error {
    fn default() -> Self {
        Self::Other
    }
}