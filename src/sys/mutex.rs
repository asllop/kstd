use core::{
    sync::atomic::{
        AtomicUsize, Ordering
    },
    cell::UnsafeCell,
    ops::{Drop, Deref}
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

    /// Acquire a lock.
    pub fn acquire(&self) -> KLock<T> {
        //TODO: we could change ordering to relaxed, and disable/enable task switching before/after fetch_add
        let q_pos = self.queue_num.fetch_add(1, Ordering::SeqCst);
        while self.current_num.load(Ordering::SeqCst) != q_pos {
            //TODO: once task switching is implemented, force switch here
        }
        KLock::new(self)
    }

    /// Release a lock
    fn release(&self) {
        // Only unlock if we are currently locked
        if self.current_num.load(Ordering::Relaxed) == self.queue_num.load(Ordering::Relaxed) - 1 {
            self.current_num.fetch_add(1, Ordering::SeqCst);
        }
    }

    /// Reset queue.
    /// 
    /// `WARNING`: Don't call it unless you know very well what you are doing!
    pub fn reset(&self) {
        self.current_num.store(0, Ordering::Relaxed);
        self.queue_num.store(0, Ordering::Relaxed);
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
        self.mutex.release();
    }
}
