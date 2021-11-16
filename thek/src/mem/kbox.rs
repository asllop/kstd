use core::{
    alloc::Layout,
    ops::Drop
};
use super::arch::ALIGN;

/// A box that allocates exact segment sizes.
pub struct KBox {
    buffer: *mut u8,
    layout: Layout
}

impl KBox {
    /// Allocates at least `size` bytes, but rounds up to the closest segment length.
    pub fn new(size: usize) -> Result<Self, ()> {
        let layout = if let Ok(l) = Layout::from_size_align(size, ALIGN) {
            l
        }
        else {
            return Err(());
        };
        //TODO: ask the mem global allocator for the closest segment size
        let buffer = unsafe { alloc::alloc::alloc(layout) };
        if buffer.is_null() {
            Err(())
        }
        else {
            Ok(
                Self {
                    buffer,
                    layout
                }
            )
        }
    }

    /// Bottom address of the allocated buffer.
    pub fn bottom(&self) -> *const u8 {
        self.buffer as *const u8
    }

    /// Top address of the allocated buffer.
    pub fn top(&self) -> *const u8 {
        unsafe {
            self.buffer.add(self.layout.size())
        }
    }

    /// Allocated buffer size.
    pub fn size(&self) -> usize {
        self.layout.size()
    } 
}

impl Drop for KBox {
    fn drop(&mut self) {
        unsafe {
            alloc::alloc::dealloc(self.buffer, self.layout);
        }
    }
}
