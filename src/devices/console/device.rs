use core::{
    marker::PhantomData
};

use crate::{
    devices::{
        console::{
            AnsiColor
        }
    },
    sys::KMutex
};

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
