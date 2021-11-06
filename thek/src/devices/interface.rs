use crate::{
    sys::{
        KMutex, KError
    },
    controllers::text::ansi::AnsiColor
};

// TODO: remove this, mutex will be implemented in the device instance handler
/// The trait that all devices must implement.
pub trait Device<'a> {
    /// Return the kernel mutex that holds the device.
    fn mutex() -> &'a KMutex<Self> where Self: Sized;
}

/// Provides an identifier.
pub trait Id {
    fn id(&self) -> &str;
    fn id_code(&self) -> usize;
}

//TODO: define an interruption trait to handle interrupts

/// Storage device interface.
pub trait StorageDevice : Id {
    /// Seek to `position` in sectors.
    /// * Return: actual position after seek.
    fn seek(&mut self, position: usize) -> Result<usize, KError>;
    /// Get current position in sectors.
    fn position(&self) -> Result<usize, KError>;
    /// Sector size in bytes.
    fn sector(&self) -> Result<usize, KError>;
    /// Read `size` sectors into `buffer`. Must be big enough to allocate size * sector_size bytes.
    /// * Return: actual bytes read.
    fn read(&self, size: usize, buffer: &mut u8) -> Result<usize, KError>;
    /// Write `size` sectors from `buffer`. Must be big enough to contain size * sector_size bytes.
    /// * Return: actual bytes written.
    fn write(&mut self, size: usize, buffer: &u8) -> Result<usize, KError>;
}

/// Text screen cursor shape.
pub enum CursorShape {
    FullBlock,
    HalfBlock,
    UnderLine,
    Default
}

/// Text screen cursor blinking.
pub enum CursorBlink {
    Fast,
    Slow,
    None,
    Default
}

/// Text mode screen device interface.
pub trait TextScreenDevice : Id {
    /// Set character at X,Y position, not changing the color.
    fn set_char(&mut self, x: usize, y: usize, ch: u8) -> Result<(), KError>;

    /// Set color at X,Y position, not changing the character.
    fn set_color(&mut self, x: usize, y: usize, text_color: AnsiColor, bg_color: AnsiColor) -> Result<(), KError>;

    /// Print one char with color at X,Y position.
    fn write(&mut self, x: usize, y: usize, text_color: AnsiColor, bg_color: AnsiColor, ch: u8) -> Result<(), KError> {
        self.set_char(x, y, ch)?;
        self.set_color(x, y, text_color, bg_color)
    }

    /// Read char and color at X,Y position, return char, text color and background color in this order.
    fn read(&self, x: usize, y: usize) -> Result<(u8, AnsiColor, AnsiColor), KError>;

    /// Set cursor X,Y position.
    fn set_position(&mut self, x: usize, y: usize) -> Result<(), KError>;

    /// Get cursor X,Y position.
    fn get_position(&self) -> Result<(usize, usize), KError>;

    /// Config cursor options.
    fn config_cursor(&mut self, enabled: bool, shape: CursorShape, blink: CursorBlink) -> Result<(), KError>;

    /// Get screen size in Columns,Rows.
    fn size(&self) -> Result<(usize, usize), KError>;
}

/// Keyset device interface.
pub trait KeysetDevice : Id {
    /// There is a key ready to be read.
    fn is_ready(&self) -> bool;
    /// Read key as processed character. Blocks if no key ready.
    fn read(&self) -> char;
    /// Read raw key code. Blocks if no key ready.
    fn read_raw(&self) -> u8;
}

/// Generic device interface.
pub trait GenericDevice : Id {
    /// Read `size` bytes into `buffer`. Must be big enough to allocate size bytes.
    /// * Return: actual bytes read.
    fn read(&self, size: usize, buffer: &mut u8) -> Result<usize, KError>;
    /// Write `size` bytes from `buffer`. Must be big enough to contain size bytes.
    /// * Return: actual bytes written.
    fn write(&mut self, size: usize, buffer: &u8) -> Result<usize, KError>;
    /// Send `command` with optional `data`.
    /// * Return: command result.
    fn cmd(&mut self, command: usize, data: Option<&u8>) -> Result<Option<&u8>, KError>;
}

/// Network device interface.
pub trait NetworkDevice : Id {
    /// Read `size` bytes into `buffer`. Must be big enough to allocate size bytes.
    /// * Return: actual bytes read.
    fn read(&self, size: usize, buffer: &mut u8) -> Result<usize, KError>;
    /// Write `size` bytes from `buffer`. Must be big enough to contain size bytes.
    /// * Return: actual bytes written.
    fn write(&mut self, size: usize, buffer: &u8) -> Result<usize, KError>;

    //TODO: config network device
}

/// UART speed enum.
pub enum UartSpeed {
    Baud110, Baud300, Baud600, Baud1200, Baud2400, Baud4800, Baud9600, Baud14400, Baud19200, Baud38400, Baud57600, Baud115200, Baud128000, Baud256000
}

/// UART parity enum.
pub enum UartParity {
    None, Even, Odd, Mark, Space 
}

/// UART port device interface.
pub trait UartPortDevice : Id {
    /// Configure the port.
    fn config(&mut self,
        parity: UartParity,
        data_bits: u8,
        stop_bits: u8,
        speed: UartSpeed) -> Result<(), KError>;
    /// Write data to port.
    fn write(&mut self, b: u8) -> Result<(), KError>;
    /// Read data from port. Blocks if not data ready.
    fn read(&self) -> Result<u8, KError>;
    /// There is data ready to be read.
    fn is_ready(&self) -> bool;
}

/*
Device System Requierements:

- Register multiple device types and multiple instances of the same type.
- Get any device using a path device_type->device_id. For example: storage->hd0, port->uart->com2
- We lock by getting a device ref, and unlock by droping it.
- Automatic registering on startup using macros.

How do we do it?

- Define the device types:
  - Storage (SATA disk, sdcard, USB pendrive, etc)
  - Network
  - TextScreen
  - GfxScreen
  - Printer
  - Keyset (keyboard, remote controller, etc)
  - Tracker (mouse, touchpad, etc)
  - Port:
      - UART
      - SPI
      - I2C
      - 1-Wire
      - USB
  - Generic: accepts reading and writing byte streams and sending commands as byte streams and reading state as byte streams.

  And then create a trait for each type and an enum to encapsulate them.
  The actual devices will continue living in static variables to avoid allocating mem for them. In the device struct we will have referenes to them.

- Questions:
  - How do we resolve the stdio problem?
       There must be controller types, not device types:
         - TextOutput
         - TextInput
       We use these controllers to hide the underlying device complexity.
*/