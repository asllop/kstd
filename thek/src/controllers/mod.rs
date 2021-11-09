//! Controllers.
//! 
//! Controllers act as the interface between applications and Devices. They offer an abstraction layer to interact with the hardware, hiding the internal details.
//! A controller may be interacting with several devices or even other controllers, but this is transparent for the user, that perceives it as a single, high level entity.
//! 
//! Controllers provide things like:
//! 
//! - Stdio
//! - File system
//! - Sockets
//! - Virtual devices like frame buffer

pub mod text;