//! Port devices.

use super::{
    Id, Interrupt
};

use crate::sys::KError;

pub mod uart;

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
    // As SPI
    fn as_spi(&self) -> Option<&dyn Spi>;
    // As I2C
    fn as_i2c(&self) -> Option<&dyn I2c>;
    // As 1-Wire
    fn as_1wire(&self) -> Option<&dyn OneWire>;
    // As USB
    fn as_usb(&self) -> Option<&dyn Usb>;
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

//TODO: implement port traits
pub trait Spi : Port {}
pub trait I2c : Port {}
pub trait OneWire : Port {}
pub trait Usb : Port {}
