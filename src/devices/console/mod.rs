use core::{
    convert::From,
    fmt::Error,
    marker::PhantomData
};

use super::{
    InputFlow, OutputFlow
};

use crate::sys::KMutex;

/**********************
 * Arch Independant
 **********************/

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

/// Console Commands
pub enum ConCmd {
    /// Print at position (X,Y) with text color and background color
    Print(usize, usize, AnsiColor, AnsiColor),
    /// Read from position (X,Y)
    Read(usize, usize),
    /// Set cursor at position (X,Y)
    SetCursor(usize, usize),
    /// Set Get cursor position
    GetCursor,
    /// Enable cursor
    EnableCursor,
    /// Disable cursor
    DisableCursor,
    /// Get console size in rows and columns
    GetSize
}

/// Console Command Result
pub enum ConCmdResult {
    /// New cursor position (X,Y)
    CursorPos(usize, usize),
    /// ASCII character with text and background colors
    Character(u8, AnsiColor, AnsiColor),
    /// No result
    None
}

impl Default for ConCmdResult {
    fn default() -> Self {
        Self::None
    }
}

type Empty = u8;

/// Console Device
/// 
/// Private field is only to prevent anyone from creating a ConsoleDevice from outside this module.
pub struct ConsoleDevice(PhantomData<Empty>);

/// Public Console Device Interface
pub static CON_DEVICE : KMutex<ConsoleDevice> = KMutex::new(ConsoleDevice(PhantomData));

//TODO: put each arch dependant ConsoleDevice implementation into a different file and compile conditionally

/**********************
 * Arch Dependant
 **********************/

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum VgaConsoleColor {
    Black = 0,
    Blue,
    Green,
    Cyan,
    Red,
    Purple,
    Brown,
    Gray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    LightPurple,
    Yellow,
    White
}

/// Convert an ANSI color to an VGA console color code
impl From<AnsiColor> for VgaConsoleColor {
    fn from(value: AnsiColor) -> Self {
        match value {
            AnsiColor::Black => VgaConsoleColor::Black,
            AnsiColor::Blue => VgaConsoleColor::Blue,
            AnsiColor::Green => VgaConsoleColor::Green,
            AnsiColor::Cyan => VgaConsoleColor::Cyan,
            AnsiColor::Red => VgaConsoleColor::Red,
            AnsiColor::Magenta => VgaConsoleColor::Purple,
            AnsiColor::Yellow => VgaConsoleColor::Brown,
            AnsiColor::BrightWhite => VgaConsoleColor::Gray,
            AnsiColor::BrightBlack => VgaConsoleColor::DarkGray,
            AnsiColor::BrightBlue => VgaConsoleColor::LightBlue,
            AnsiColor::BrightGreen => VgaConsoleColor::LightGreen,
            AnsiColor::BrightCyan => VgaConsoleColor::LightCyan,
            AnsiColor::BrightRed => VgaConsoleColor::LightRed,
            AnsiColor::BrightMagenta => VgaConsoleColor::LightPurple,
            AnsiColor::BrightYellow => VgaConsoleColor::Yellow,
            AnsiColor::White => VgaConsoleColor::White,
            // First 16 colors are the ones supported by VgaConsoleColor. For the rest we map in blocks of 16.
            AnsiColor::Color256(c) => {
                if c < 16 {
                    VgaConsoleColor::from(c)
                }
                else {
                    VgaConsoleColor::from(c / 16u8)
                }
            }
        }
    }
}

impl From<u8> for VgaConsoleColor {
    fn from(value: u8) -> Self {
        match value & 0xF {
            0 => VgaConsoleColor::Black,
            1 => VgaConsoleColor::Blue,
            2 => VgaConsoleColor::Green,
            3 => VgaConsoleColor::Cyan,
            4 => VgaConsoleColor::Red,
            5 => VgaConsoleColor::Purple,
            6 => VgaConsoleColor::Brown,
            7 => VgaConsoleColor::Gray,
            8 => VgaConsoleColor::DarkGray,
            9 => VgaConsoleColor::LightBlue,
            10 => VgaConsoleColor::LightGreen,
            11 => VgaConsoleColor::LightCyan,
            12 => VgaConsoleColor::LightRed,
            13 => VgaConsoleColor::LightPurple,
            14 => VgaConsoleColor::Yellow,
            15 => VgaConsoleColor::White,
            _ => VgaConsoleColor::Black
        }
    }
}

// TODO: implement remaining commands for ConsoleDevice.

/// For commands without data, like SetCursor and Enable/DisableCursor, we implement InputFlow with unit type.
impl InputFlow<()> for ConsoleDevice {
    type Command = ConCmd;
    type CmdResult = ConCmdResult;

    fn write_cmd(&self, cmd: Self::Command, data: ()) -> Result<Self::CmdResult, Error> {
        match cmd {
            ConCmd::SetCursor(x, y) => {
                //TODO
                Ok(ConCmdResult::default())
            },
            ConCmd::EnableCursor => {
                //TODO
                Ok(ConCmdResult::default())
            },
            ConCmd::DisableCursor => {
                //TODO
                Ok(ConCmdResult::default())
            },
            _ => Err(Error)
        }
    }
}

/// Print a single ASCII char
impl InputFlow<u8> for ConsoleDevice {
    type Command = ConCmd;
    type CmdResult = ConCmdResult;

    fn write_cmd(&self, cmd: Self::Command, data: u8) -> Result<Self::CmdResult, Error> {
        match cmd {
            ConCmd::Print(x, y, text_color, bg_color) => {
                let pos = 80 * y + x;
                if pos < 2000 {
                    unsafe {
                        *((0xB8000 + pos * 2) as *mut u8) = data;
                        let color = ((VgaConsoleColor::from(bg_color) as u8) << 4) | (VgaConsoleColor::from(text_color) as u8);
                        *((0xB8000 + pos * 2 + 1) as *mut u8) = color;
                    }
                    //TODO: move cursor to new location or simply return the new location as result
                    Ok(ConCmdResult::default())
                }
                else {
                    Err(Error)
                }
            },
            _ => Err(Error)
        }
    }
}

/// Print an array of ASCII chars
impl InputFlow<&[u8]> for ConsoleDevice {
    type Command = ConCmd;
    type CmdResult = ConCmdResult;

    fn write_cmd(&self, cmd: Self::Command, data: &[u8]) -> Result<Self::CmdResult, Error> {
        match cmd {
            ConCmd::Print(mut x, mut y, text_color, bg_color) => {
                for ch in data {
                    self.write_cmd(ConCmd::Print(x, y, text_color, bg_color), *ch)?;
                    x += 1;
                    if x >= 80 {
                        x = 0;
                        y += 1;
                    }
                    if y >= 25 {
                        y = 0;
                    }
                }
                Ok(ConCmdResult::default())
            },
            _ => Err(Error)
        }
    }
}
