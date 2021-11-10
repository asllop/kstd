//! Types for hardware access.

pub mod text;

pub mod port;

pub mod storage;

pub mod network;

pub mod generic;

pub mod keyset;

mod interface;
pub use interface::*;
