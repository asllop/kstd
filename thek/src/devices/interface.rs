use core::sync::atomic::AtomicBool;

use crate::{
    sys::{
        KMutex, KLock
    }
};

use hashbrown::{
    HashMap, hash_map::DefaultHashBuilder
};

use macros::register_devices;

use super::{
    storage::Storage,
    generic::Generic,
    keyset::Keyset,
    network::Network,
    text::Text,
    port::Port
};

/// Device store.
struct DeviceStore {
    storage: HashMap<&'static str, Device>,
    text: HashMap<&'static str, Device>,
    keyset: HashMap<&'static str, Device>,
    network: HashMap<&'static str, Device>,
    port: HashMap<&'static str, Device>,
    generic: HashMap<&'static str, Device>
}

macro_rules! def_device_map {
    () => {
        // We have to manually create a HashMap (with hardcoded seeds) because it doesn't provide a const constructor.
        HashMap::with_hasher(
            DefaultHashBuilder::with_seeds(
                103428633845345,
                4723874528374,
                5332938,
                3847737465837
            )
        )
    };
}

impl DeviceStore {
    const fn new() -> Self {
        Self {
            storage: def_device_map!(),
            text: def_device_map!(),
            keyset: def_device_map!(),
            network: def_device_map!(),
            port: def_device_map!(),
            generic: def_device_map!()
        }
    }

    fn get(&self, store: &HashMap<&'static str, Device>, id: &str) -> Option<Device> {
        if let Some(&device) = store.get(id) {
            Some(device)
        }
        else {
            None
        }
    }

    /// Get a Storage device by ID.
    pub fn get_storage(&self, id: &str) -> Option<Device> {
        self.get(&self.storage, id)
    }
    
    /// Remove a Storage device by ID.
    pub fn remove_storage(&mut self, id: &str) -> bool {
        self.storage.remove(id).is_some()
    }

    /// Get a Text device by ID.
    pub fn get_text(&self, id: &str) -> Option<Device> {
        self.get(&self.text, id)
    }
    
    /// Remove a Text device by ID.
    pub fn remove_text(&mut self, id: &str) -> bool {
        self.text.remove(id).is_some()
    }

    /// Get a Keyset device by ID.
    pub fn get_keyset(&self, id: &str) -> Option<Device> {
        self.get(&self.keyset, id)
    }
    
    /// Remove a Keyset device by ID.
    pub fn remove_keyset(&mut self, id: &str) -> bool {
        self.keyset.remove(id).is_some()
    }

    /// Get a Network device by ID.
    pub fn get_network(&self, id: &str) -> Option<Device> {
        self.get(&self.network, id)
    }
    
    /// Remove a Network device by ID.
    pub fn remove_network(&mut self, id: &str) -> bool {
        self.network.remove(id).is_some()
    }

    /// Get a Port device by ID.
    pub fn get_port(&self, id: &str) -> Option<Device> {
        self.get(&self.port, id)
    }
    
    /// Remove a Port device by ID.
    pub fn remove_port(&mut self, id: &str) -> bool {
        self.port.remove(id).is_some()
    }

    /// Get a Generic device by ID.
    pub fn get_generic(&self, id: &str) -> Option<Device> {
        self.get(&self.generic, id)
    }
    
    /// Remove a Generic device by ID.
    pub fn remove_generic(&mut self, id: &str) -> bool {
        self.generic.remove(id).is_some()
    }
    
    /// Register device.
    pub fn register_device(&mut self, device: Device) -> bool {
        match device {
            Device::Storage(m) => {
                self.storage.insert(m.acquire().id(), device);
                true
            },
            Device::Text(m) => {
                self.text.insert(m.acquire().id(), device);
                true
            },
            Device::Keyset(m) => {
                self.keyset.insert(m.acquire().id(), device);
                true
            },
            Device::Network(m) => {
                self.network.insert(m.acquire().id(), device);
                true
            },
            Device::Port(m) => {
                self.port.insert(m.acquire().id(), device);
                true
            },
            Device::Generic(m) => {
                self.generic.insert(m.acquire().id(), device);
                true
            }
        }
    }
}

/// Get a Storage device by ID.
pub fn get_storage_device(id: &str) -> Option<Device> {
    let _lock = DEVICE_STORE_MUTEX.acquire();
    unsafe {
        DEVICE_STORE.get_storage(id)
    }
}

/// Get a Text device by ID.
pub fn get_text_device(id: &str) -> Option<Device> {
    let _lock = DEVICE_STORE_MUTEX.acquire();
    unsafe {
        DEVICE_STORE.get_text(id)
    }
}

/// Get a Keyset device by ID.
pub fn get_keyset_device(id: &str) -> Option<Device> {
    let _lock = DEVICE_STORE_MUTEX.acquire();
    unsafe {
        DEVICE_STORE.get_keyset(id)
    }
}

/// Get a Network device by ID.
pub fn get_network_device(id: &str) -> Option<Device> {
    let _lock = DEVICE_STORE_MUTEX.acquire();
    unsafe {
        DEVICE_STORE.get_network(id)
    }
}

/// Get a Port device by ID.
pub fn get_port_device(id: &str) -> Option<Device> {
    let _lock = DEVICE_STORE_MUTEX.acquire();
    unsafe {
        DEVICE_STORE.get_port(id)
    }
}

/// Get a Generic device by ID.
pub fn get_generic_device(id: &str) -> Option<Device> {
    let _lock = DEVICE_STORE_MUTEX.acquire();
    unsafe {
        DEVICE_STORE.get_generic(id)
    }
}

/// Unregister a device.
pub fn unregister_device(device: Device) -> bool {
    let _lock = DEVICE_STORE_MUTEX.acquire();
    unsafe {
        match device {
            Device::Storage(m) => {
                DEVICE_STORE.remove_storage(m.acquire().id())
            },
            Device::Text(m) => {
                DEVICE_STORE.remove_text(m.acquire().id())
            },
            Device::Keyset(m) => {
                DEVICE_STORE.remove_keyset(m.acquire().id())
            },
            Device::Network(m) => {
                DEVICE_STORE.remove_network(m.acquire().id())
            },
            Device::Port(m) => {
                DEVICE_STORE.remove_port(m.acquire().id())
            },
            Device::Generic(m) => {
                DEVICE_STORE.remove_generic(m.acquire().id())
            }
        }
    }
}

/// Register a device.
pub fn register_device(device: Device) -> bool {
    let _lock = DEVICE_STORE_MUTEX.acquire();
    unsafe {
        DEVICE_STORE.register_device(device)
    }
}

#[register_devices("thek/src/devices")]
pub fn init_devices() {}

static mut DEVICE_STORE : DeviceStore = DeviceStore::new();
static DEVICE_STORE_MUTEX : KMutex<AtomicBool> = KMutex::new(AtomicBool::new(true));

/// Encapsulate all device types.
#[derive(Clone, Copy)]
pub enum Device {
    /// Storage devices (hard disks, memory cards, ...)
    Storage(&'static KMutex<&'static dyn Storage>),
    /// Text output devices (text screens, serial consoles, ...)
    Text(&'static KMutex<&'static dyn Text>),
    /// Keyset input devices (keyboard, remote controller, ...)
    Keyset(&'static KMutex<&'static dyn Keyset>),
    /// Network devices (ethernet, token ring, slip, ppp, ...)
    Network(&'static KMutex<&'static dyn Network>),
    /// Port devices (UART, USB, SPI, ...)
    Port(&'static KMutex<&'static dyn Port>),
    /// Generic devices. Whatever that is not covered by the other types.
    Generic(&'static KMutex<&'static dyn Generic>)
}

impl Device {
    /// Force storage unwrap and lock device.
    pub fn unwrap_storage(&self) -> KLock<'_, &'static dyn Storage> {
        if let Device::Storage(m) = self {
            m.acquire()
        }
        else {
            panic!("Not a Storage device");
        }
    }
    
    /// Force text unwrap and lock device.
    pub fn unwrap_text(&self) -> KLock<'_, &'static dyn Text> {
        if let Device::Text(m) = self {
            m.acquire()
        }
        else {
            panic!("Not a Text device");
        }
    }

    /// Force keyset unwrap and lock device.
    pub fn unwrap_keyset(&self) -> KLock<'_, &'static dyn Keyset> {
        if let Device::Keyset(m) = self {
            m.acquire()
        }
        else {
            panic!("Not a Keyset device");
        }
    }

    /// Force network unwrap and lock device.
    pub fn unwrap_network(&self) -> KLock<'_, &'static dyn Network> {
        if let Device::Network(m) = self {
            m.acquire()
        }
        else {
            panic!("Not a Network device");
        }
    }

    /// Force port unwrap and lock device.
    pub fn unwrap_port(&self) -> KLock<'_, &'static dyn Port> {
        if let Device::Port(m) = self {
            m.acquire()
        }
        else {
            panic!("Not a Port device");
        }
    }

    /// Force generic unwrap and lock device.
    pub fn unwrap_generic(&self) -> KLock<'_, &'static dyn Generic> {
        if let Device::Generic(m) = self {
            m.acquire()
        }
        else {
            panic!("Not a Generic device");
        }
    }
}

/// Provides an identifier.
pub trait Id {
    /// Device identifier.
    /// By convention, 3 capital letters followed by a 1-based index.
    ///
    /// Commonly used device IDs:
    /// - `SER` for UART ports (e.g., SER1)
    /// - `HDD` for hard disks (e.g., HDD1 is the primary hard disk)
    /// - `KBD` for keyboards (e.g., KBD7)
    /// - `CON` for text consoles (e.g., CON1 is the default text output, usually the screen)
    /// - `ETH` for ethernet cards (e.g., ETH2)
    fn id(&self) -> &str;
}

/// Device interrupts.
pub trait Interrupt {
    /// Set an interrupt handler.
    /// * Return: could be set or not.
    fn handler(&self, func: fn(device: Device)) -> bool;
}

/*
Other device types we could define:
  - Gfx (2D and 3D)
  - Printer
  - Tracker (mouse, touchpad, touch screen, etc)
*/