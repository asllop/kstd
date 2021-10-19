use core::{
    sync::atomic::{
        AtomicUsize, Ordering
    },
    cell::UnsafeCell,
    ops::{Drop, Deref, DerefMut}
};

/// Kernel mutex with queuing.
pub struct KMutex<T> {
    queue_num: AtomicUsize,
    current_num: AtomicUsize,
    host: UnsafeCell<T>
}

unsafe impl<T: Sized + Send> Sync for KMutex<T> {}

impl<T> KMutex<T> {
    /// Create new mutex.
    pub const fn new(host: T) -> Self {
        Self {
            queue_num: AtomicUsize::new(0),
            current_num: AtomicUsize::new(0),
            host: UnsafeCell::new(host)
        }
    }

    /// Return a lock.
    pub fn lock(&self) -> KLock<T> {
        //TODO: we could change ordering to relaxed, and disable7enable task switching before/after fetch_add
        let q_pos = self.queue_num.fetch_add(1, Ordering::SeqCst);
        while self.current_num.load(Ordering::SeqCst) != q_pos {
            //TODO: once task switching is implemented, force switch here
        }
        KLock::new(self)
    }

    fn unlock(&self) {
        self.current_num.fetch_add(1, Ordering::SeqCst);
    }
}

/// Lock returned by [`KMutex::lock()`].
/// 
/// It's a smart pointer that gives access to inner type.
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

//TODO: to have mutable ref we need an UnsafeCell on self.mutex ref, otherwise it is an immutable ref.
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
