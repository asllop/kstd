use alloc::alloc::{GlobalAlloc, Layout};
use core::{
    ptr::null_mut,
    sync::atomic::{
        AtomicUsize, Ordering
    },
};
use super::{
    layout::MemBlockSet,
    arch::raw_mem
};
use crate::sys::KMutex;

extern crate alloc;

/// Memory Global Allocator
struct Memory;

impl Memory {
    unsafe fn get_block_set(&self) -> &mut MemBlockSet {
        let (mem_ptr, _) = raw_mem();
        &mut*(mem_ptr as *mut MemBlockSet)
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
            if let Err(e) = block_layout.push_address(ptr) {
                panic!("Could not push address into segment stack: {}", e.msg());
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
                // Call default implementation of realloc on a different type, otherwise we would get a recursive infinite loop
                Super::realloc(&AUX_M, ptr, layout, new_size)
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

/// Awful trick used to avoid reimplementing GlobalAlloc::realloc and to emulate a call to "super" (trait default implementation of realloc).
struct _M;

type Super = _M;

unsafe impl GlobalAlloc for _M {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        GLOB_ALLOC.alloc(_layout)
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        GLOB_ALLOC.dealloc(_ptr, _layout)
    }
}

static AUX_M : _M = _M;

/// Memory allocator resource mutex
/// 
/// The counter is for both, keep track of the currently allocated segments and to prevent rust from optimizing out the lock variable if it's not used.
static MEM_MUTEX : KMutex<AtomicUsize> = KMutex::new(AtomicUsize::new(0));

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Memory allocation error: {:?}", layout)
}
