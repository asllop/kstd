//! Network devices.

use super::{
    Id, Interrupt
};

use crate::sys::KError;

/// Network type.
pub enum NetworkType {
    Loopback,
    Ethernet,
    Slip,
    Ppp,
    TokenRing
}

/// Network device interface.
pub trait Network : Id + Interrupt {
    /// Read `size` bytes into `buffer`. Must be big enough to allocate size bytes.
    /// * Return: actual bytes read.
    fn read(&self, size: usize, buffer: &mut u8) -> Result<usize, KError>;
    /// Write `size` bytes from `buffer`. Must be big enough to contain size bytes.
    /// * Return: actual bytes written.
    fn write(&self, size: usize, buffer: &u8) -> Result<usize, KError>;
    /// Network type.
    fn net_type(&self) -> NetworkType;
    /// As Ethernet.
    fn as_eth(&self) -> Option<&dyn EthernetNetwork>;
    /// As SLIP.
    fn as_slip(&self) -> Option<&dyn SlipNetwork>;
    //TODO: conversion to other network types
}

/// Ethernet interface
pub trait EthernetNetwork : Network {
    /// TODO: configure an ethernet network interface
    fn config(&self);
}

/// SLIP interface
pub trait SlipNetwork : Network {
    /// TODO: configure a slip network interface
    fn config(&self);
}
