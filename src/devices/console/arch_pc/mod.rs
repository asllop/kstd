use core::{
    convert::From
};

use crate::{
    devices::{
        InputFlow,
        console::{
            AnsiColor, ConCmd, ConCmdResult, ConsoleDevice
        }
    },
    sys::Error
};

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
            ConCmd::Read(x, y) => {
                //TODO
                Ok(ConCmdResult::default())
            },
            ConCmd::GetCursor => {
                //TODO
                Ok(ConCmdResult::default())
            },
            ConCmd::GetSize => {
                //TODO
                Ok(ConCmdResult::default())
            },
            _ => Err(Error::WrongCmd)
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
                    Err(Error::BufOutBounds)
                }
            },
            _ => Err(Error::WrongCmd)
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
            _ => Err(Error::WrongCmd)
        }
    }
}