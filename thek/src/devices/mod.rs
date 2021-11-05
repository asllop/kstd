//! Abstraction types for direct hardware access.

pub mod plot;

pub mod com;

use crate::sys::KMutex;

/// The trait that all devices must implement.
pub trait Device<'a> {
    /// Return the kernel mutex that holds the device.
    fn mutex() -> &'a KMutex<Self> where Self: Sized;
}

/*
Device System Requierements:

- Register multiple device types and multiple instances of the same type.
- Get any device using a path device_type->device_id. For example: storage->hd0, port->uart->com2
- We lock by getting a device ref, and unlock by droping it.
- Automatic registering on startup using macros.

How do we do it?

- Define the device types:
  - Storage (SATA disk, sdcard, USB pendrive, etc)
  - Network
  - TextScreen
  - GfxScreen
  - Printer
  - Keyset (keyboard, remote controller, etc)
  - Tracker (mouse, touchpad, etc)
  - Port:
      - UART
      - SPI
      - I2C
      - 1-Wire
      - USB
  - Generic: accepts reading and writing byte streams and sending commands as byte streams and reading state as byte streams.

  And then create a trait for each type and an enum to encapsulate them.
  The actual devices will continue living in static variables to avoid allocating mem for them. In the device struct we will have referenes to them.

- Questions:
  - How do we resolve the stdio problem?
       There must be controller types, not device types:
         - TextOutput
         - TextInput
       We use these controllers to hide the underlying device complexity.
*/