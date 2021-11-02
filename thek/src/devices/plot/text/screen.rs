use core::{
    marker::PhantomData
};

use crate::sys::{
    KMutex, Void
};

use super::super::super::Device;

/// Screen text device.
/// 
/// Can't be directly instantiated.
pub struct ScreenTextDevice(Void);

impl Device<'_> for ScreenTextDevice {
    fn mutex() -> &'static KMutex<Self> {
        &SCREEN_TEXT_DEVICE
    }
}

/// Screen device static instance.
static SCREEN_TEXT_DEVICE : KMutex<ScreenTextDevice> = KMutex::new(ScreenTextDevice(PhantomData));
