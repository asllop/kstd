use core::{
    marker::PhantomData
};

use crate::{
    devices::{
        console::{
            AnsiColor
        }
    },
    sys::{
        KMutex as Mutex, KError as Error
    }
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
    Pos(usize, usize),
    /// ASCII character with text and background colors
    Character(u8, AnsiColor, AnsiColor),
    /// Console size in Columns, Rows
    Size(usize, usize),
    /// No result
    None
}

impl Default for ConCmdResult {
    fn default() -> Self {
        Self::None
    }
}

// Why the cmd system is better than a static interface like the following?
// With commands, one arch can give more features that are available to only that arch
/*
pub trait ConsoleDeviceInterface {
    fn print(x: usize, y: usize, tex_color: AnsiColor, bg_color: AnsiColor, ascii: u8) -> Result<(), Error>;
    fn print_array(x: usize, y: usize, tex_color: AnsiColor, bg_color: AnsiColor, ascii_str: &[u8]) -> Result<(), Error>;
    fn read(x: usize, y: usize) -> Result<(u8, AnsiColor, AnsiColor), Error>;
    fn set_cursor(x: usize, y: usize) -> Result<(), Error>;
    fn get_cursor() -> Result<(usize, usize), Error>;
    fn enable_cursor() -> Result<(), Error>;
    fn disable_cursor() -> Result<(), Error>;
    fn get_size() -> Result<(usize, usize), Error>;
}
*/

type Empty = u8;

/// Console Device
/// 
/// Private field is only to prevent anyone from creating a ConsoleDevice from outside this module.
pub struct ConsoleDevice(PhantomData<Empty>);

/// Public Console Device Interface
pub static CON_DEVICE : Mutex<ConsoleDevice> = Mutex::new(ConsoleDevice(PhantomData));
