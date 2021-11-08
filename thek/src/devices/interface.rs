use crate::{
    sys::{
        KMutex, KLock, KError
    },
    controllers::text::ansi::AnsiColor
};

use hashbrown::{
    HashMap, hash_map::DefaultHashBuilder
};

// TODO: remove this, mutex will be implemented in the device instance handler
/// The trait that all devices must implement.
pub trait Device<'a> {
    /// Return the kernel mutex that holds the device.
    fn mutex() -> &'a KMutex<Self> where Self: Sized;
}

pub fn get_storage(id: &str) -> Option<DeviceType> {
    if let Some(&device_type) = STORAGE_DEVICES.acquire().get(id) {
        Some(device_type)
    }
    else {
        None
    }
}

pub fn remove_storage(id: &str) -> bool {
    if let Some(_) = STORAGE_DEVICES.acquire().remove(id) {
        true
    }
    else {
        false
    }
}

pub fn register_device(device_type: DeviceType) -> bool {
    match device_type {
        DeviceType::Storage(m) => {
            STORAGE_DEVICES.acquire().insert(m.acquire().id(), device_type);
            true
        },
        //TODO: implement register for the rest of device types
        _ => false
    }
}

type DeviceStore = KMutex<HashMap<&'static str, DeviceType>>;

// We have to manually create a HashMap (with hardcoded seeds) because it doesn't provide a const constructor.

static STORAGE_DEVICES : DeviceStore = KMutex::new(
    HashMap::with_hasher(
    DefaultHashBuilder::with_seeds(103428633845345, 4723874528374, 5318798732938, 3847737465837)
    )
);

//TODO: interface to store devices:
// - to obtain devices we want something like get_TYPE(ID): get_storage("HDA"), get_port("COM1"), etc.
// - static hashmap per device type. The key is the ID and the value the DeviceType.
// - when a device is get, it locks it automatically using KMutex.
// - devices can be dynamically added and removed.

/// Encapsulate all device types.
#[derive(Clone, Copy)]
pub enum DeviceType {
    Storage(&'static KMutex<&'static dyn Storage>),
    Text(&'static KMutex<&'static dyn Text>),
    Keyset(&'static KMutex<&'static dyn Keyset>),
    Network(&'static KMutex<&'static dyn Network>),
    Port(&'static KMutex<&'static dyn Port>),
    Generic(&'static KMutex<&'static dyn Generic>)
}

impl DeviceType {
    pub fn unwrap_storage(&self) -> KLock<'_, &'static dyn Storage> {
        if let DeviceType::Storage(m) = self {
            m.acquire()
        }
        else {
            panic!("Not a Storage device");
        }
    }
    //TODO: unwrap the rest of device types
}

/*
struct TestDev {}
impl BuildDevice for TestDev {
    fn build_device() -> DeviceType {
        DeviceType::Generic(&_MUTEX)
    }
}
impl Generic for TestDev {
    fn read(&self, _: usize, _: &mut u8) -> Result<usize, KError> {
        Err(KError::Other)
    }

    fn write(&mut self, _: usize, _: &u8) -> Result<usize, KError> {
        Err(KError::Other)
    }

    fn cmd(&mut self, _: usize, _: Option<&u8>) -> Result<Option<&u8>, KError> {
        Err(KError::Other)
    }
}
impl Id for TestDev {
    fn id(&self) -> &str { "TST0" }
}
impl Interrupt for TestDev {
    fn handler(&self, _: fn(DeviceType)) -> bool { false }
}
static _DEVICE : TestDev = TestDev {};
static _MUTEX : KMutex<&'static dyn Generic> = KMutex::new(&_DEVICE);
*/

/// Build a DeviceType enum.
pub trait BuildDevice {
    fn build_device() -> DeviceType;
}

/// Provides an identifier.
pub trait Id {
    // Device identifier (e.g., COM1, HDA, ETH0, etc).
    fn id(&self) -> &str;
}

/// Device interrupts.
pub trait Interrupt {
    /// Set an interrupt handler.
    /// * Return: could be set or not.
    fn handler(&self, func: fn(device: DeviceType)) -> bool;
}

//TODO: define an interruption trait to handle interrupts

/// Storage device interface.
pub trait Storage : Id + Interrupt {
    /// Seek to `position` in sectors.
    /// * Return: actual position after seek.
    fn seek(&self, position: usize) -> Result<usize, KError>;
    /// Get current position in sectors.
    fn position(&self) -> Result<usize, KError>;
    /// Sector size in bytes.
    fn sector(&self) -> Result<usize, KError>;
    /// Read `size` sectors into `buffer`. Must be big enough to allocate size * sector_size bytes.
    /// * Return: actual bytes read.
    fn read(&self, size: usize, buffer: &mut u8) -> Result<usize, KError>;
    /// Write `size` sectors from `buffer`. Must be big enough to contain size * sector_size bytes.
    /// * Return: actual bytes written.
    fn write(&self, size: usize, buffer: &u8) -> Result<usize, KError>;
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

/// Text device interface.
pub trait Text : Id + Interrupt {
    /// Put a character at X,Y position, not changing the color.
    fn put_char(&self, x: usize, y: usize, ch: char) -> Result<(), KError>;
    /// Put color at X,Y position, not changing the character.
    fn put_color(&self, x: usize, y: usize, text_color: AnsiColor, bg_color: AnsiColor) -> Result<(), KError>;
    /// Print one char with color at X,Y position.
    fn write(&self, x: usize, y: usize, text_color: AnsiColor, bg_color: AnsiColor, ch: char) -> Result<(), KError> {
        self.put_char(x, y, ch)?;
        self.put_color(x, y, text_color, bg_color)
    }
    /// Read char and color at X,Y position, return char, text color and background color in this order.
    fn read(&self, x: usize, y: usize) -> Result<(char, AnsiColor, AnsiColor), KError>;
    /// Set cursor X,Y position.
    fn set_position(&self, x: usize, y: usize) -> Result<(), KError>;
    /// Get cursor X,Y position.
    fn get_position(&self) -> Result<(usize, usize), KError>;
    /// Config cursor options.
    fn config_cursor(&self, enabled: bool, shape: CursorShape, blink: CursorBlink) -> Result<(), KError>;
    /// Get screen size in Columns,Rows.
    fn size(&self) -> Result<(usize, usize), KError>;
}

/// Processed char.
pub enum KeyChar {
    Press(char),
    Release(char)
}

/// Keyset device interface.
pub trait Keyset : Id + Interrupt {
    /// There is a key ready to be read.
    fn is_ready(&self) -> bool;
    /// Read raw key code. Blocks if no key ready.
    fn read(&self) -> u8;
    /// Read key as a processed character. Blocks if no key ready.
    fn char_read(&self) -> KeyChar;
}

/// Network type.
pub enum NetworkType {
    Loopback,
    Ethernet,
    Slip,
    Ppp,
    TokenRing
}

/// Network device interface.
pub trait Network : Id + Interrupt {
    /// Read `size` bytes into `buffer`. Must be big enough to allocate size bytes.
    /// * Return: actual bytes read.
    fn read(&self, size: usize, buffer: &mut u8) -> Result<usize, KError>;
    /// Write `size` bytes from `buffer`. Must be big enough to contain size bytes.
    /// * Return: actual bytes written.
    fn write(&self, size: usize, buffer: &u8) -> Result<usize, KError>;
    /// Network type.
    fn net_type(&self) -> NetworkType;
    /// As Ethernet.
    fn as_eth(&self) -> Option<&dyn EthernetNetwork>;
    /// As SLIP.
    fn as_slip(&self) -> Option<&dyn SlipNetwork>;
    //TODO: conversion to other network types
}

/// Ethernet interface
pub trait EthernetNetwork : Network {
    /// TODO: configure an ethernet network interface
    fn config(&self);
}

/// SLIP interface
pub trait SlipNetwork : Network {
    /// TODO: configure a slip network interface
    fn config(&self);
}

/// Port type.
pub enum PortType {
    Uart,
    Spi,
    I2c,
    OneWire,
    Usb
}

pub trait Port : Id + Interrupt {
    /// Write data to port.
    fn write(&self, b: u8) -> Result<(), KError>;
    /// Read data from port. Blocks if not data ready.
    fn read(&self) -> Result<u8, KError>;
    /// There is data ready to be read.
    fn is_ready(&self) -> bool;
    /// Port type.
    fn port_type(&self) -> PortType;
    /// As UART.
    fn as_uart(&self) -> Option<&dyn Uart>;
    //TODO: other port conversions
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
pub trait Uart : Port {
    /// Configure the port.
    fn config(&self,
        parity: UartParity,
        data_bits: u8,
        stop_bits: u8,
        speed: UartSpeed) -> Result<(), KError>;
}

/// Generic device interface.
pub trait Generic : Id + Interrupt {
    /// Read `size` bytes into `buffer`. Must be big enough to allocate size bytes.
    /// * Return: actual bytes read.
    fn read(&self, size: usize, buffer: &mut u8) -> Result<usize, KError>;
    /// Write `size` bytes from `buffer`. Must be big enough to contain size bytes.
    /// * Return: actual bytes written.
    fn write(&self, size: usize, buffer: &u8) -> Result<usize, KError>;
    /// Send `command` with optional `data`.
    /// * Return: command result.
    fn cmd(&self, command: usize, data: Option<&u8>) -> Result<Option<&u8>, KError>;
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