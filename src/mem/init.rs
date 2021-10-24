//! Memory initializations.

use super::{
    layout::MemBlockLayout,
    arch::raw_mem
};

use core::mem::size_of;

/// 4K segment size
const SEGMENT_SIZE : usize = 4*1024;

/// Initialize memory structures
pub fn setup_mem() {
    unsafe {
        init_mem();
    }
}

/// We set only one block with 4K segments, for better performance we should set multiple blocks with different segment size
unsafe fn init_mem() {
    let (mem_ptr, mem_size, _align) = raw_mem();

    let block_base_address = mem_ptr.add(size_of::<MemBlockLayout>());
    let block_size = mem_size - size_of::<MemBlockLayout>();
    let num_segments = block_size / SEGMENT_SIZE;
    let stack_size_bytes = num_segments * size_of::<*mut u8>();
    let payload_address = block_base_address.add(stack_size_bytes);
    // Pointer to pointers
    let block_base_address = block_base_address as *mut *mut u8;

    // Create stack with pointers to all segments in the block
    for segment_index in 0..num_segments {
        let offset = segment_index * SEGMENT_SIZE;
        *(block_base_address.add(segment_index)) = payload_address.add(offset);
    }

    // Create layout struct
    let block_layout = MemBlockLayout {
        stack_ptr: block_base_address,
        payload_ptr: payload_address as *mut u8,
        segment_size: SEGMENT_SIZE,
        num_segments,
        used_segments: 0
    };

    *((mem_ptr) as *mut MemBlockLayout) = block_layout;
}