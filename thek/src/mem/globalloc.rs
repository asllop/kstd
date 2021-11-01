use alloc::alloc::{GlobalAlloc, Layout};
use core::{
    ptr::null_mut,
    sync::atomic::{
        AtomicUsize, Ordering
    },
};
use super::{
    layout::{
        MemBlockSet
    },
    arch::raw_mem
};
use crate::sys::{
    KMutex
};

extern crate alloc;

/// Memory Global Allocator
struct Memory;

impl Memory {
    unsafe fn get_block_set(&self) -> &mut MemBlockSet {
        let (mem_ptr, _) = raw_mem();
        &mut*(mem_ptr as *mut MemBlockSet)
    }

    /// Reimplement GlobalAlloc::realloc :(
    fn _realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: the caller must ensure that the `new_size` does not overflow.
        // `layout.align()` comes from a `Layout` and is thus guaranteed to be valid.
        let new_layout = unsafe { Layout::from_size_align_unchecked(new_size, layout.align()) };
        // SAFETY: the caller must ensure that `new_layout` is greater than zero.
        let new_ptr = unsafe { self.alloc(new_layout) };
        if !new_ptr.is_null() {
            // SAFETY: the previously allocated block cannot overlap the newly allocated block.
            // The safety contract for `dealloc` must be upheld by the caller.
            unsafe {
                core::intrinsics::copy_nonoverlapping(ptr, new_ptr, core::cmp::min(layout.size(), new_size));
                self.dealloc(ptr, layout);
            }
        }
        new_ptr
    }
}

unsafe impl GlobalAlloc for Memory {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let lock = MEM_MUTEX.acquire();
        let block_set = self.get_block_set();
        if let Some(block_layout) = block_set.find_block(layout.size()) {
            if layout.size() < block_layout.segment_size {
                if let Some(segment_ptr) = block_layout.pop_address() {
                    lock.fetch_add(1, Ordering::Relaxed);
                    segment_ptr
                }
                else {
                    null_mut()
                }
            }
            else {
                null_mut()
            }
        }
        else {
            null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let lock = MEM_MUTEX.acquire();
        let block_set = self.get_block_set();
        if let Some(block_layout) = block_set.owns_segment(ptr) {
            if let Err(_) = block_layout.push_address(ptr) {
                panic!("Could not push address into segment stack");
            }
            lock.fetch_sub(1, Ordering::Relaxed);
        }
        else {
            panic!("Could not find a block that owns the segment {:#x}", ptr as usize);
        }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let lock = MEM_MUTEX.acquire();
        let block_set = self.get_block_set();
        if let Some(block_layout) = block_set.owns_segment(ptr) {
            // If new size fits within the current memory segment, reuse it
            if new_size <= block_layout.segment_size {
                ptr
            }
            else {
                core::mem::drop(lock);
                self._realloc(ptr, layout, new_size)
            }
        }
        else {
            panic!("Could not find a block that owns the segment {:#x}", ptr as usize);
        }
    }
}

/// Global Allocator static instance
#[global_allocator]
static GLOB_ALLOC : Memory = Memory;

/// Memory allocator resource mutex
/// 
/// The counter is for both, keep track of the currently allocated segments and to prevent rust from optimizing out the lock variable if it's not used.
static MEM_MUTEX : KMutex<AtomicUsize> = KMutex::new(AtomicUsize::new(0));

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Memory allocation error: {:?}", layout)
}
