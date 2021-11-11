//! Character and ANSI utils.

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

/// Convert into character.
pub trait IntoChar {
    type Error;
    fn into_char(&self) -> Result<char, Self::Error>;
}

// TODO: use different encodings
/// Convert char into ASCII character (8 bits, assuming Latin-1 encoding).
impl IntoAscii for char {
    type Error = KError;
    fn into_ascii(&self) -> Result<u8, Self::Error> {
        Ok(*self as u8)
    }
}

// TODO: use different encodings
/// Convert u8 ASCII into character (8 bits, assuming Latin-1 encoding).
impl IntoChar for u8 {
    type Error = KError;
    fn into_char(&self) -> Result<char, Self::Error> {
        Ok(*self as char)
    }
}
