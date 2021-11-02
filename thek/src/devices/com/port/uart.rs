//! UART port device.

pub enum UartSpeed {
    Baud110, Baud300, Baud600, Baud1200, Baud2400, Baud4800, Baud9600, Baud14400, Baud19200, Baud38400, Baud57600, Baud115200, Baud128000, Baud256000
}

pub enum UartParity {
    None, Even, Odd, Mark, Space 
}

use core::default::Default;
use core::fmt::{
    Write, Error
};

/// Serial (UART) device
pub struct UartDevice {
    port_number: u8,
    parity: UartParity,
    data_bits: u8,
    stop_bits: u8,
    speed: UartSpeed
}

impl UartDevice {
    /// New device with config
    pub fn new(
        port_number: u8,
        parity: UartParity,
        data_bits: u8,
        stop_bits: u8,
        speed: UartSpeed) -> Self {
            let _self = Self {
                port_number, parity, data_bits, stop_bits, speed
            };
            _self.init_port();
            _self
    }

    /// Send string over the wire.
    pub fn send_str(&self, s: &str) {
        for ch in s.as_bytes() {
            self.send(*ch);
        }
    }
}

impl Default for UartDevice {
    fn default() -> Self {
        Self::new(0, UartParity::None, 8, 1, UartSpeed::Baud9600)
    }
}

//TODO: mutex

//TODO: move to a controller
impl Write for UartDevice {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        self.send_str(s);
        Ok(())
    }
}