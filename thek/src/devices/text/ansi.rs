//! Character utils and ANSI related module.

use crate::sys::KError;

/// Define an ANSI color
/// 
/// More info about ANSI colors: <https://en.wikipedia.org/wiki/ANSI_escape_code#Colors>
#[derive(Copy, Clone)]
pub enum AnsiColor {
    // Basic terminals
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    // Bright-Bold terminals
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    // Extended color terminals
    Color256(u8)
}

/// Convert into ASCII character.
pub trait IntoAscii {
    type Error;
    fn into_ascii(&self) -> Result<u8, Self::Error>;
}

/// Convert char into ASCII character.
impl IntoAscii for char {
    type Error = KError;
    fn into_ascii(&self) -> Result<u8, Self::Error> {
        let b = (*self as u32 & 0x7F) as u8;
        if b as char == *self {
            Ok(b)
        }
        else {
            //TODO: create error
            Err(KError::Other)
        }
    }
}