use core::{
    convert::From,
    fmt::Error
};

use super::{
    InputFlow, OutputFlow
};

/**********************
 * Arch Independant
 **********************/

/// Define an ANSI color (https://www.lihaoyi.com/post/BuildyourownCommandLinewithANSIescapecodes.html)
#[derive(Copy, Clone)]
pub enum AnsiColor {
    // Basic termimals
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

/// Console Device
pub struct ConsoleDevice;

/// Public Console Device Interface
pub static CON_DEVICE : ConsoleDevice = ConsoleDevice;

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
            AnsiColor::Color256(_) => VgaConsoleColor::Black
        }
    }
}

// TODO: implement remaining commands for ConsoleDevice.

impl InputFlow<Option<u8>> for ConsoleDevice {
    type Command = ConCmd;

    fn write_cmd(&self, cmd: Self::Command, data: Option<u8>) -> Result<(), Error> {
        match cmd {
            ConCmd::Print(x, y, text_color, bg_color) => {
                if let Some(data) = data {
                    let pos = 80 * y + x;
                    if pos < 2000 {
                        unsafe {
                            *((0xB8000 + pos * 2) as *mut u8) = data;
                            let color = ((VgaConsoleColor::from(bg_color) as u8) << 4) | (VgaConsoleColor::from(text_color) as u8);
                            *((0xB8000 + pos * 2 + 1) as *mut u8) = color;
                        }
                        Ok(())
                    }
                    else {
                        Err(Error)
                    }
                }
                else {
                    Err(Error)
                }
            },
            _ => Err(Error)
        }
    }
}

impl InputFlow<&[u8]> for ConsoleDevice {
    type Command = ConCmd;

    fn write_cmd(&self, cmd: Self::Command, data: &[u8]) -> Result<(), Error> {
        match cmd {
            ConCmd::Print(mut x, mut y, text_color, bg_color) => {
                for ch in data {
                    self.write_cmd(ConCmd::Print(x, y, text_color, bg_color), Some(*ch))?;
                    x += 1;
                    if x >= 80 {
                        x = 0;
                        y += 1;
                    }
                    if y >= 25 {
                        y = 0;
                    }
                }
                Ok(())
            },
            _ => Err(Error)
        }
    }
}
