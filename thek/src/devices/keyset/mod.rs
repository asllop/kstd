//! Keyset devices.

use super::{
    Id, Interrupt
};

/// Processed char.
pub enum KeyChar {
    Press(char),
    Release(char)
}

/// Keyset device interface.
pub trait Keyset : Id + Interrupt {
    /// There is a key ready to be read.
    fn is_ready(&self) -> bool;
    /// Read raw key code. Blocks if no key ready.
    fn read(&self) -> u8;
    /// Read key as a processed character. Blocks if no key ready.
    fn char_read(&self) -> KeyChar;
}
