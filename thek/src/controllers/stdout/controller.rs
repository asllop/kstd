use alloc::boxed::Box;

use crate::sys::KMutex;

use core::{
    fmt::{
        Write,
        Result,
        Error
    },
    ops::{
        Deref, DerefMut
    }
};

use crate::controllers::text::TextController;

/// Stdout controller. It's a wrapper to other controllers implementing [`core::fmt::Write`][`Write`].
pub struct StdoutController;

impl StdoutController {
    pub const fn new() -> Self {
        StdoutController
    }

    /// Set the stdout controller.
    pub fn set(val: Box<dyn Write>) {
        let lock = STDOUT.acquire();
        let cell = lock.get_host();
        unsafe {
            *cell.get() = Some(val);
        }
    }
}

impl Deref for StdoutController {
    type Target = Option<Box<dyn Write>>;

    fn deref(&self) -> &Self::Target {
        let lock = STDOUT.acquire();
        let cell = lock.get_host();
        unsafe {
            &*cell.get()
        }
    }
}

impl DerefMut for StdoutController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let lock = STDOUT.acquire();
        let cell = lock.get_host();
        unsafe {
            &mut *cell.get()
        }
    }
}

impl Default for StdoutController {
    fn default() -> Self {
        let _self = Self::new();
        if let None = _self.as_deref() {
            Self::set(Box::new(TextController::default()));
        }
        _self
    }
}

impl Write for StdoutController {
    fn write_str(&mut self, s: &str) -> Result {
        if let Some(rf) = self.as_deref_mut() {
            rf.write_str(s)
        }
        else {
            Err(Error)
        }
    }
}

static STDOUT : KMutex<Option<Box<dyn Write>>> = KMutex::new(None);
