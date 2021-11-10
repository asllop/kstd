use core::{
    sync::atomic::{
        AtomicUsize, Ordering
    },
    cell::UnsafeCell,
    ops::{Drop, Deref, DerefMut},
};

/// Kernel mutex with queuing.
pub struct KMutex<T> {
    queue_num: AtomicUsize,
    current_num: AtomicUsize,
    host: UnsafeCell<T>
}

unsafe impl<T: Sized> Sync for KMutex<T> {}

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
    pub fn reset(&self) -> &Self {
        self.current_num.store(0, Ordering::Relaxed);
        self.queue_num.store(0, Ordering::Relaxed);
        self
    }
}

/// Lock returned by [`KMutex::acquire()`].
/// 
/// It's a smart pointer that gives mutable access to the inner value.
pub struct KLock<'a, T> {
    mutex_ref: &'a KMutex<T>,
    host_ref: &'a mut T
}

impl<'a, T> KLock<'a, T> {
    fn new(mutex_ref: &'a KMutex<T>) -> Self {
        Self {
            mutex_ref,
            host_ref: unsafe { &mut *mutex_ref.host.get() }
        }
    }

    /// Access to internal unsafe cell.
    pub fn get_host(&self) -> &UnsafeCell<T> {
        &self.mutex_ref.host
    }
}

impl<'a, T> Deref for KLock<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.host_ref
    }
}

impl<'a, T> DerefMut for KLock<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.host_ref
    }
}

impl<'a, T> Drop for KLock<'a, T> {
    fn drop(&mut self) {
        self.mutex_ref.release();
    }
}
