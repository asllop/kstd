use core::{
    fmt::{
        Write,
        Error
    },
    default::Default
};

use crate::devices::{
    self, Device
};

use crate::sys::KError;

use alloc::{
    borrow::ToOwned,
    string::String
};


/// Port controller.
pub struct PortController {
    device_id: String
}

impl PortController {
    fn get_device(id: &str) -> Result<Device, KError> {
        if let Some(d) = devices::get_port_device(id) {
            Ok(d)
        }
        else {
            Err(KError::Other)
        }
    }
    
    /// We assume the port device is already configured.
    pub fn new(device_id: String) -> Result<Self, KError> {
        Ok(
            Self {
                device_id
            }
        )
    }
}

impl Default for PortController {
    fn default() -> Self {
        Self::new("SER1".to_owned()).expect("Error creating serial controller")
    }
}

impl Write for PortController {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        if let Ok(device) = Self::get_device(&self.device_id) {
            let port_dev = device.unwrap_port();
            for ch in s.as_bytes() {
                port_dev.write(*ch).unwrap_or_default();
            }
            Ok(())
        }
        else {
            Err(Error)
        }
    }
}
