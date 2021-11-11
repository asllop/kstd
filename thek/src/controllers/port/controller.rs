use core::{
    fmt::{
        Write,
        Error
    },
    default::Default
};

use crate::devices::{
    self, Device,
    port::{
        UartParity, UartSpeed
    }
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
    
    /// We assume the `device_id` is a port device and is already configured.
    pub fn new(device_id: String) -> Self {
        Self {
            device_id
        }
    }

    /// Create new controller for UART with port config.
    pub fn from_uart(
        device_id: String,
        parity: UartParity,
        data_bits: u8,
        stop_bits: u8,
        speed: UartSpeed
    ) -> Result<Self, KError> {
        if let Ok(device) = Self::get_device(&device_id) {
            let port_dev = device.unwrap_port();
            if let Some(port_dev) = port_dev.as_uart() {
                port_dev.config(parity, data_bits, stop_bits, speed)?;
                return Ok(
                    Self {
                        device_id
                    }
                );
            }
        }
        Err(KError::Other)
    }

    //TODO: create "from" constructors for other port types.
}

impl Default for PortController {
    fn default() -> Self {
        Self::new("SER1".to_owned())
    }
}

impl Write for PortController {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        if let Ok(device) = Self::get_device(&self.device_id) {
            let port_dev = device.unwrap_port();
            //TODO: iter as chars and convert into appropiate encoding (configured in the kernel). Otherwise we are always using UTF-8, the str type encoding.
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
