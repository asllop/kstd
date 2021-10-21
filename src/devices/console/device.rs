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
        KMutex, Void
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

// Q: Why the cmd system is better than a static interface (a trait with defined functions)?
// A: With commands, one arch can give more features that are available to only that arch. We have more flexibility. We also don't need to care about mutable/immutable refs.

/// Console Device
/// 
/// Can't be directly instantiated.
pub struct ConsoleDevice(Void);

/// Public Console Device Interface
pub static CON_DEVICE : KMutex<ConsoleDevice> = KMutex::new(ConsoleDevice(PhantomData));
