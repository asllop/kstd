/// Define an ANSI color
/// 
/// More info about ANSI colors: https://en.wikipedia.org/wiki/ANSI_escape_code#Colors
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