//! PC COM port device (UART port).

use crate::arch::{
    inb, outb
};

use crate::sys::{
    KMutex, KError
};

use crate::devices::{
    Id, Interrupt, Port, PortType, Uart, UartParity, UartSpeed, DeviceType, DeviceStore
};

//TODO: add macro mark
pub fn register_devices(device_store: &KMutex<DeviceStore>) {
    device_store.acquire().register_device(DeviceType::Port(&PC_COM_DEVICE_1_MUTEX));
}

static PC_COM_DEVICE_1 : PcComDevice = PcComDevice::new(0);
static PC_COM_DEVICE_1_MUTEX : KMutex<&'static dyn Port> = KMutex::new(&PC_COM_DEVICE_1);

/// PC COM device.
pub struct PcComDevice {
    port_number: u8
}

impl Port for PcComDevice {
    fn write(&self, b: u8) -> Result<(), KError> {
        let port = self.port_addr();
        while !self.is_transmit_empty() {}
        outb(port,b);
        Ok(())
    }

    fn read(&self) -> Result<u8, KError> {
        while !self.is_ready() {}
        let port = self.port_addr();
        Ok(inb(port))
    }

    fn is_ready(&self) -> bool {
        let port = self.port_addr();
        inb(port + 5) & 1 != 0
    }

    fn port_type(&self) -> PortType {
        PortType::Uart
    }

    fn as_uart(&self) -> Option<&dyn Uart> {
        Some(self as &dyn Uart)
    }

    fn as_spi(&self) -> Option<&dyn crate::devices::Spi> { None }
    fn as_i2c(&self) -> Option<&dyn crate::devices::I2c> { None }
    fn as_1wire(&self) -> Option<&dyn crate::devices::OneWire> { None }
    fn as_usb(&self) -> Option<&dyn crate::devices::Usb> { None }
}

impl Uart for PcComDevice {
    fn config(&self,
        _parity: UartParity,
        _data_bits: u8,
        _stop_bits: u8,
        _speed: UartSpeed) -> Result<(), KError> {

        let port = self.port_addr();
        //TODO: use config data to init. Now using hardcoded config.
        outb(port + 1, 0x00);    // Disable all interrupts
        outb(port + 3, 0x80);    // Enable DLAB (set baud rate divisor)
        outb(port + 0, 0x03);    // Set divisor to 3 (lo byte) 38400 baud
        outb(port + 1, 0x00);    //                  (hi byte)
        outb(port + 3, 0x03);    // 8 bits, no parity, one stop bit
        outb(port + 2, 0xC7);    // Enable FIFO, clear them, with 14-byte threshold
        outb(port + 4, 0x0B);    // IRQs enabled, RTS/DSR set
        outb(port + 4, 0x1E);    // Set in loopback mode, test the serial chip
        outb(port + 0, 0xAE);    // Test serial chip (send byte 0xAE and check if serial returns same byte)
      
        // Check if serial is faulty (i.e: not same byte as sent)
        if inb(port + 0) != 0xAE {
            //TODO: define a KError
           Err(KError::Other)
        }
        else {
            // If serial is not faulty set it in normal operation mode
            // (not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled)
            outb(port + 4, 0x0F);
            Ok(())
        }
    }
}

impl Id for PcComDevice {
    fn id(&self) -> &str {
        match self.port_number {
            0 => "COM1",
            1 => "COM2",
            2 => "COM3",
            3 => "COM4",
            _ => "COM1"
        }
    }
}

impl Interrupt for PcComDevice {
    //TODO: setup ints
    fn handler(&self, _func: fn(DeviceType)) -> bool { false }
}

impl PcComDevice {
    const fn new(port_number: u8) -> Self {
        Self {
            port_number
        }
    }

    fn is_transmit_empty(&self) -> bool {
        let port = self.port_addr();
        inb(port + 5) & 0x20 != 0
    }
    
    fn port_addr(&self) -> u16 {
        match self.port_number {
            // COM1
            0 => 0x3F8,
            // COM2
            1 => 0x2F8,
            // COM3
            2 => 0x3E8,
            // COM4
            3 => 0x2E8,
            // Default COM1
            _ => 0x3F8
        }
    }
}