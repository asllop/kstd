//! UART port device.

use core::{
    fmt::{
        Write, Error
    }
};
use super::super::super::Device;
use crate::sys::KMutex;

pub enum UartSpeed {
    Baud110, Baud300, Baud600, Baud1200, Baud2400, Baud4800, Baud9600, Baud14400, Baud19200, Baud38400, Baud57600, Baud115200, Baud128000, Baud256000
}

pub enum UartParity {
    None, Even, Odd, Mark, Space 
}

/// Serial (UART) device
pub struct UartDevice {
    pub port_number: u8,
    pub parity: UartParity,
    pub data_bits: u8,
    pub stop_bits: u8,
    pub speed: UartSpeed
}

impl UartDevice {
    /// New device with config.
    pub const fn new(
        port_number: u8,
        parity: UartParity,
        data_bits: u8,
        stop_bits: u8,
        speed: UartSpeed) -> Self {
            Self {
                port_number, parity, data_bits, stop_bits, speed
            }
    }

    /// Configure the port.
    pub fn config(&mut self, port_number: u8,
        parity: UartParity,
        data_bits: u8,
        stop_bits: u8,
        speed: UartSpeed) {
            self.port_number = port_number;
            self.parity = parity;
            self.data_bits = data_bits;
            self.stop_bits = stop_bits;
            self.speed = speed;
            self.init_port();
    }
}

impl Device<'_> for UartDevice {
    fn mutex() -> &'static KMutex<Self> {
        &UART_DEVICE
    }
}

//TODO: register multiple UART ports (COM1, COM2, etc).
static UART_DEVICE : KMutex<UartDevice> = KMutex::new(UartDevice::new(
    0,
    UartParity::None,
    8,
    1,
    UartSpeed::Baud9600
));

//TODO: move to a controller
impl Write for UartDevice {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        for ch in s.as_bytes() {
            self.send(*ch);
        }
        Ok(())
    }
}
