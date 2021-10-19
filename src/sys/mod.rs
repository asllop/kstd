use core::{
    sync::atomic::{
        AtomicBool, Ordering
    },
    cell::UnsafeCell,
    ops::{Drop, Deref, DerefMut}
};

/// Simple spinlock mutex.
pub struct KMutex<T> {
    is_locked: AtomicBool,
    host: UnsafeCell<T>
}

unsafe impl<T: Sized + Send> Sync for KMutex<T> {}

impl<T> KMutex<T> {
    /// Create new mutex.
    pub const fn new(host: T) -> Self {
        Self {
            is_locked: AtomicBool::new(false),
            host: UnsafeCell::new(host)
        }
    }

    /// Return a lock guard.
    pub fn lock(&self) -> KLock<T> {
        loop {
            let res = self.is_locked.compare_exchange(
                false,
                true,
                Ordering::SeqCst,
                Ordering::Acquire
            );

            if let Ok(_) = res {
                break;
            }
        }
        KLock::new(self)
    }

    fn unlock(&self) {
        self.is_locked.store(false, Ordering::SeqCst);
    }
}

/// Lock guard returned by [`KMutex`].
pub struct KLock<'a, T> {
    mutex: &'a KMutex<T>
}

impl<'a, T> KLock<'a, T> {
    const fn new(mutex: &'a KMutex<T>) -> Self {
        Self {
            mutex
        }
    }
}

impl<'a, T> Deref for KLock<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { 
            &*self.mutex.host.get()
        }
    }
}

/*
impl<'a, T> DerefMut for KLock<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { 
            &mut *self.mutex.host.get_mut()
        }
    }
}
*/

impl<'a, T> Drop for KLock<'a, T> {
    fn drop(&mut self) {
        self.mutex.unlock();
    }
}
