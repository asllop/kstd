use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use super::{
    layout::MemBlockLayout,
    arch::raw_mem
};

extern crate alloc;

/// Memory Global Allocator
pub struct Memory;

unsafe impl GlobalAlloc for Memory {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        let (mem_ptr, _, _) = raw_mem();
        let block_layout = &mut*(mem_ptr as *mut MemBlockLayout);
        if _layout.size() < block_layout.segment_size {
            if let Some(segment_ptr) = block_layout.pop_address() {
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

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        let (mem_ptr, _, _) = raw_mem();
        let block_layout = &mut*(mem_ptr as *mut MemBlockLayout);
        if let Err(_) = block_layout.push_address(_ptr) {
            panic!("dealloc should be never called")
        }
    }
}

/// Global Allocator static instance
#[global_allocator]
static GLOB_ALLOC : Memory = Memory;

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Memory allocation error: {:?}", layout)
}
