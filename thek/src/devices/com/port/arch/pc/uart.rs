//! UART port implementation for PC.

use super::super::super::super::port::uart::UartDevice;
use crate::arch::{
    inb, outb
};

impl UartDevice {
    /// Init port.
    pub fn init_port(&self) {
        //TODO: use port data to init. Now using hardcoded config.
        let port = 0x3f8;
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
           return;
        }
      
        // If serial is not faulty set it in normal operation mode
        // (not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled)
        outb(port + 4, 0x0F);
    }

    fn is_transmit_empty(&self) -> u8 {
        let port = 0x3f8;
        return inb(port + 5) & 0x20;
    }
      
    /// Send char
    pub fn send(&self, b: u8) {
        let port = 0x3f8;
        while self.is_transmit_empty() == 0 {}
        outb(port,b);
    }

    //TODO: receive method
}