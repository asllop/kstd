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

/// Device store.
pub struct DeviceStore {
    storage: HashMap<&'static str, DeviceType>,
    text: HashMap<&'static str, DeviceType>,
    keyset: HashMap<&'static str, DeviceType>,
    network: HashMap<&'static str, DeviceType>,
    port: HashMap<&'static str, DeviceType>
}

impl DeviceStore {
    // We have to manually create a HashMap (with hardcoded seeds) because it doesn't provide a const constructor.
    const fn new() -> Self {
        Self {
            storage: HashMap::with_hasher(
                DefaultHashBuilder::with_seeds(
                103428633845345,
                4723874528374,
                5318798732938,
                3847737465837
                )
            ),
            text: HashMap::with_hasher(
                DefaultHashBuilder::with_seeds(
                103428633845345,
                4723874528374,
                5318798732938,
                3847737465837
                )
            ),
            keyset: HashMap::with_hasher(
                DefaultHashBuilder::with_seeds(
                103428633845345,
                4723874528374,
                5318798732938,
                3847737465837
                )
            ),
            network: HashMap::with_hasher(
                DefaultHashBuilder::with_seeds(
                103428633845345,
                4723874528374,
                5318798732938,
                3847737465837
                )
            ),
            port: HashMap::with_hasher(
                DefaultHashBuilder::with_seeds(
                103428633845345,
                4723874528374,
                5318798732938,
                3847737465837
                )
            )
        }
    }

    fn get(&self, store: &HashMap<&'static str, DeviceType>, id: &str) -> Option<DeviceType> {
        if let Some(&device_type) = store.get(id) {
            Some(device_type)
        }
        else {
            None
        }
    }

    pub fn get_storage(&self, id: &str) -> Option<DeviceType> {
        self.get(&self.storage, id)
    }
    
    pub fn remove_storage(&mut self, id: &str) -> bool {
        self.storage.remove(id).is_some()
    }

    pub fn get_text(&self, id: &str) -> Option<DeviceType> {
        self.get(&self.text, id)
    }
    
    pub fn remove_text(&mut self, id: &str) -> bool {
        self.text.remove(id).is_some()
    }

    pub fn get_keyset(&self, id: &str) -> Option<DeviceType> {
        self.get(&self.keyset, id)
    }
    
    pub fn remove_keyset(&mut self, id: &str) -> bool {
        self.keyset.remove(id).is_some()
    }

    pub fn get_network(&self, id: &str) -> Option<DeviceType> {
        self.get(&self.network, id)
    }
    
    pub fn remove_network(&mut self, id: &str) -> bool {
        self.network.remove(id).is_some()
    }

    pub fn get_port(&self, id: &str) -> Option<DeviceType> {
        self.get(&self.port, id)
    }
    
    pub fn remove_port(&mut self, id: &str) -> bool {
        self.port.remove(id).is_some()
    }
    
    pub fn register_device(&mut self, device_type: DeviceType) -> bool {
        match device_type {
            DeviceType::Storage(m) => {
                self.storage.insert(m.acquire().id(), device_type);
                true
            },
            DeviceType::Text(m) => {
                self.text.insert(m.acquire().id(), device_type);
                true
            },
            DeviceType::Keyset(m) => {
                self.keyset.insert(m.acquire().id(), device_type);
                true
            },
            DeviceType::Network(m) => {
                self.network.insert(m.acquire().id(), device_type);
                true
            },
            DeviceType::Port(m) => {
                self.port.insert(m.acquire().id(), device_type);
                true
            },
            _ => false
        }
    }
}

pub static DEVICE_STORE : KMutex<DeviceStore> = KMutex::new(DeviceStore::new());

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
pub fn register_devices(device_store: &KMutex<DeviceStore>) {
    device_store.acquire().register_device(DeviceType::Generic(&_MUTEX));
}
pub struct TestDev;
impl Generic for TestDev {
    fn read(&self, _: usize, _: &mut u8) -> Result<usize, KError> {
        Err(KError::Other)
    }

    fn write(&self, _: usize, _: &u8) -> Result<usize, KError> {
        Err(KError::Other)
    }

    fn cmd(&self, _: usize, _: Option<&u8>) -> Result<Option<&u8>, KError> {
        Err(KError::Other)
    }
}
impl Id for TestDev {
    fn id(&self) -> &str { "TST0" }
}
impl Interrupt for TestDev {
    fn handler(&self, _: fn(DeviceType)) -> bool { false }
}
static _DEVICE : TestDev = TestDev;
static _MUTEX : KMutex<&'static dyn Generic> = KMutex::new(&_DEVICE);
*/

/// Provides an identifier.
pub trait Id {
    // Device identifier.
    // By convention, 3 capital letters followed by a number (e.g., COM1, HDD12, ETH0).
    fn id(&self) -> &str;
}

/// Device interrupts.
pub trait Interrupt {
    /// Set an interrupt handler.
    /// * Return: could be set or not.
    fn handler(&self, func: fn(device: DeviceType)) -> bool;
}

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
Other device types we could define:
  - Gfx (2D and 3D)
  - Printer
  - Tracker (mouse, touchpad, touch screen, etc)
*/