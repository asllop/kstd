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
        let num_segs = NUM_SEGS.acquire();
        let used_mem = USED_MEM.acquire();

        let block_set = self.get_block_set();
        if let Some(block_layout) = block_set.find_block(layout.size()) {
            if layout.size() < block_layout.segment_size {
                if let Some(segment_ptr) = block_layout.pop_address() {
                    num_segs.fetch_add(1, Ordering::Relaxed);
                    used_mem.fetch_add(layout.size(), Ordering::Relaxed);
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

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let num_segs = NUM_SEGS.acquire();
        let used_mem = USED_MEM.acquire();

        let block_set = self.get_block_set();
        if let Some(block_layout) = block_set.owns_segment(ptr) {
            if let Err(e) = block_layout.push_address(ptr) {
                panic!("Could not push address into segment stack: {}", e.msg());
            }
            num_segs.fetch_sub(1, Ordering::Relaxed);
            used_mem.fetch_sub(layout.size(), Ordering::Relaxed);
        }
        else {
            panic!("Could not find a block that owns the segment {:#x}", ptr as usize);
        }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let num_segs = NUM_SEGS.acquire();
        let used_mem = USED_MEM.acquire();

        let block_set = self.get_block_set();
        if let Some(block_layout) = block_set.owns_segment(ptr) {
            // If new size fits within the current memory segment, reuse it
            if new_size <= block_layout.segment_size {
                used_mem.fetch_add(new_size - layout.size(), Ordering::Relaxed);
                ptr
            }
            else {
                core::mem::drop(num_segs);
                core::mem::drop(used_mem);
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

/// Number of segments used. It acts as both, a counter for statistics and a lock to acquire the Memory resource.
static NUM_SEGS : KMutex<AtomicUsize> = KMutex::new(AtomicUsize::new(0));

/// Actual memory used. It acts as both, a counter for statistics and a lock to acquire the Memory resource.
static USED_MEM : KMutex<AtomicUsize> = KMutex::new(AtomicUsize::new(0));

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    //TODO: stop task scheduling, once we have multitasking
    let block_set = unsafe { GLOB_ALLOC.get_block_set() };
    let mut total_num_segments = 0;
    let mut total_mem = 0;
    for i in 0..block_set.num_blocks {
        if let Some(block) = block_set.block_at(i) {
            total_mem += block.segment_size * block.num_segments;
            total_num_segments += block.num_segments;
        }
    }
    let used_num_segments = NUM_SEGS.reset().acquire().load(Ordering::Relaxed);
    let used_mem = USED_MEM.reset().acquire().load(Ordering::Relaxed);
    panic!(
        "Memory allocation error: {:?} | Segments allocated: {}/{}({:.2}%) | Memory used: {}/{}({:.2}%)",
        layout,
        used_num_segments,
        total_num_segments,
        (used_num_segments as f64 / total_num_segments as f64) * 100.0,
        used_mem,
        total_mem,
        (used_mem as f64 / total_mem as f64) * 100.0
    );
}
