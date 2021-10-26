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
