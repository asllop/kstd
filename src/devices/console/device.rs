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
        KMutex, KError, Void
    }
};

/// Console device interface trait. All console devices must implement it.
pub trait ConsoleDeviceApi {

    /// Print one char with color at X,Y position
    fn print(&self, x: usize, y: usize, text_color: AnsiColor, bg_color: AnsiColor, ch: u8) -> Result<(), KError>;

    /// Print a string with color into X,Y position
    fn print_array(&self, mut x: usize, mut y: usize, text_color: AnsiColor, bg_color: AnsiColor, ch_array: &[u8]) -> Result<(), KError> {
        let (cols, rows) = self.get_size()?;
        for ch in ch_array {
            self.print(x, y, text_color, bg_color, *ch)?;
            x += 1;
            if x >= cols {
                x = 0;
                y += 1;
            }
            if y >= rows {
                y = 0;
            }
        }
        Ok(())
    }

    /// Set color at X,Y position
    fn set_color(&self, x: usize, y: usize, text_color: AnsiColor, bg_color: AnsiColor) -> Result<(), KError>;

    /// Read char and color at X,Y position
    fn read(&self, x: usize, y: usize) -> Result<(u8, AnsiColor, AnsiColor), KError>;

    /// Set cursor X,Y position
    fn set_cursor(&self, x: usize, y: usize) -> Result<(), KError>;

    /// Get cursor X,Y position
    fn get_cursor(&self) -> Result<(usize, usize), KError>;

    /// Enable cursor (set visible)
    fn enable_cursor(&self) -> Result<(), KError>;

    /// Disable cursor (set invisible)
    fn disable_cursor(&self) -> Result<(), KError>;

    /// Get console size in Columns,Rows
    fn get_size(&self) -> Result<(usize, usize), KError>;
}

/// Console Device
/// 
/// Can't be directly instantiated.
pub struct ConsoleDevice(Void);

/// Public Console Device Interface
pub static CON_DEVICE : KMutex<ConsoleDevice> = KMutex::new(ConsoleDevice(PhantomData));
