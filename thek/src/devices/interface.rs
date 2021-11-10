use core::sync::atomic::AtomicBool;

use crate::{
    sys::{
        KMutex, KLock
    }
};

use hashbrown::{
    HashMap, hash_map::DefaultHashBuilder
};

use super::{
    storage::Storage,
    generic::Generic,
    keyset::Keyset,
    network::Network,
    text::Text,
    port::Port
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
    port: HashMap<&'static str, DeviceType>,
    generic: HashMap<&'static str, DeviceType>
}

macro_rules! def_device_map {
    () => {
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
    // We have to manually create a HashMap (with hardcoded seeds) because it doesn't provide a const constructor.
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

    fn get(&self, store: &HashMap<&'static str, DeviceType>, id: &str) -> Option<DeviceType> {
        if let Some(&device_type) = store.get(id) {
            Some(device_type)
        }
        else {
            None
        }
    }

    /// Get a Storage device by ID.
    pub fn get_storage(&self, id: &str) -> Option<DeviceType> {
        self.get(&self.storage, id)
    }
    
    /// Remove a Storage device by ID.
    pub fn remove_storage(&mut self, id: &str) -> bool {
        self.storage.remove(id).is_some()
    }

    /// Get a Text device by ID.
    pub fn get_text(&self, id: &str) -> Option<DeviceType> {
        self.get(&self.text, id)
    }
    
    /// Remove a Text device by ID.
    pub fn remove_text(&mut self, id: &str) -> bool {
        self.text.remove(id).is_some()
    }

    /// Get a Keyset device by ID.
    pub fn get_keyset(&self, id: &str) -> Option<DeviceType> {
        self.get(&self.keyset, id)
    }
    
    /// Remove a Keyset device by ID.
    pub fn remove_keyset(&mut self, id: &str) -> bool {
        self.keyset.remove(id).is_some()
    }

    /// Get a Network device by ID.
    pub fn get_network(&self, id: &str) -> Option<DeviceType> {
        self.get(&self.network, id)
    }
    
    /// Remove a Network device by ID.
    pub fn remove_network(&mut self, id: &str) -> bool {
        self.network.remove(id).is_some()
    }

    /// Get a Port device by ID.
    pub fn get_port(&self, id: &str) -> Option<DeviceType> {
        self.get(&self.port, id)
    }
    
    /// Remove a Port device by ID.
    pub fn remove_port(&mut self, id: &str) -> bool {
        self.port.remove(id).is_some()
    }

    /// Get a Generic device by ID.
    pub fn get_generic(&self, id: &str) -> Option<DeviceType> {
        self.get(&self.generic, id)
    }
    
    /// Remove a Generic device by ID.
    pub fn remove_generic(&mut self, id: &str) -> bool {
        self.generic.remove(id).is_some()
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
            DeviceType::Generic(m) => {
                self.generic.insert(m.acquire().id(), device_type);
                true
            }
        }
    }
}

/// Acquire the device store.
pub fn get_device_store<'a>() -> &'a DeviceStore {
    unsafe {
        &DEVICE_STORE
    }
}

/// Register a device.
pub fn register_device(device_type: DeviceType) -> bool {
    let _lock = DEVICE_STORE_WRITE_MUTEX.acquire();
    unsafe {
        DEVICE_STORE.register_device(device_type)
    }
}

static mut DEVICE_STORE : DeviceStore = DeviceStore::new();
static DEVICE_STORE_WRITE_MUTEX : KMutex<AtomicBool> = KMutex::new(AtomicBool::new(true));

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
    
    pub fn unwrap_text(&self) -> KLock<'_, &'static dyn Text> {
        if let DeviceType::Text(m) = self {
            m.acquire()
        }
        else {
            panic!("Not a Text device");
        }
    }

    pub fn unwrap_keyset(&self) -> KLock<'_, &'static dyn Keyset> {
        if let DeviceType::Keyset(m) = self {
            m.acquire()
        }
        else {
            panic!("Not a Keyset device");
        }
    }

    pub fn unwrap_network(&self) -> KLock<'_, &'static dyn Network> {
        if let DeviceType::Network(m) = self {
            m.acquire()
        }
        else {
            panic!("Not a Network device");
        }
    }

    pub fn unwrap_port(&self) -> KLock<'_, &'static dyn Port> {
        if let DeviceType::Port(m) = self {
            m.acquire()
        }
        else {
            panic!("Not a Port device");
        }
    }

    pub fn unwrap_generic(&self) -> KLock<'_, &'static dyn Generic> {
        if let DeviceType::Generic(m) = self {
            m.acquire()
        }
        else {
            panic!("Not a Generic device");
        }
    }
}

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

/*
Other device types we could define:
  - Gfx (2D and 3D)
  - Printer
  - Tracker (mouse, touchpad, touch screen, etc)
*/