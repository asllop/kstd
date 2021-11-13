//! Memory initializations.

use super::{
    layout::{
        MemBlockLayout, MemBlockSet, MAX_NUM_BLOCKS
    },
    arch::{
        raw_mem, ALIGN
    }
};

use core::mem::size_of;

/// 4K segment size
const DEFAULT_SEGMENT_SIZE : usize = 4*1024;

/// Initialize memory structures.
/// 
/// Divide the memory in N blocks (max 5) of specified segment size (in bytes) and % of the memory occupied by the block (the first and second tuple positions respectively).
/// The segment sizes must be sorted in ascending order, and the sum of all % must be 100, otherwise it will panic.
/// Alignment is not adjusted in segments, only in blocks, so the user is responsable for choosing a segment size that is a multiple of the architecture alignment.
/// 
/// # Example
/// 
/// ```
/// // Divide memory in 4 blocks: 50% of 4KB, 25% of 16KB, 13% of 256KB, and the remaining 12% in one single segment.
/// setup_mem(&[(4*1024, 50), (16*1024, 25), (256*1024, 13), (usize::MAX, 12)]);
/// ```
pub fn setup_mem(schema: &[(usize, u8)]) {
    let schema = if schema.len() == 0 {
        &[(DEFAULT_SEGMENT_SIZE, 100)]
    }
    else {
        schema
    };

    if schema.len() > MAX_NUM_BLOCKS {
        panic!("Number of blocks can't be bigger than {} constant", MAX_NUM_BLOCKS);
    }

    let mut sum = 0;
    let mut last_size = 0;
    for (size, percentage) in schema {
        sum += percentage;
        if *size <= last_size {
            panic!("Blocks must be ordered from smaller segment size to bigger");
        }
        last_size = *size;
    }

    if sum != 100 {
        panic!("Sum of block size percentage must be 100");
    }

    unsafe {
        init_mem(schema);
    }
}

unsafe fn init_mem(schema: &[(usize, u8)]) {
    let (mem_ptr, mem_size) = raw_mem();

    // Generate MemBlockSet struct
    let mut block_set = MemBlockSet {
        block_layouts: core::mem::MaybeUninit::uninit().assume_init(),
        num_blocks: schema.len()
    };
    // Fill the array with empty layouts
    for i in 0..MAX_NUM_BLOCKS {
        block_set.block_layouts[i] = MemBlockLayout::empty();
    }

    let mut block_ptr = mem_ptr.add(size_of::<MemBlockSet>());
    let mem_size = mem_size - size_of::<MemBlockSet>();

    // Fill the valid block layouts
    for i in 0..schema.len() {
        let mut block_size = (mem_size * schema[i].1 as usize) / 100;
        // Adjust alignment in block size
        block_size -= block_size % ALIGN;

        if block_ptr as usize % ALIGN != 0 {
            panic!("Bad alignment in block {} -> {} {:#x}", i, block_ptr as usize, block_size);
        }

        // Init block
        block_set.block_layouts[i] = init_block(block_ptr, block_size, schema[i].0);
        // Recalculate block starting address
        block_ptr = block_ptr.add(block_size);
    }

    // Store block set struct
    *((mem_ptr) as *mut MemBlockSet) = block_set;
}

unsafe fn init_block(block_base_address: *mut u8, block_size: usize, segment_size: usize) -> MemBlockLayout {
    
    /*
    block_size = stack_size_bytes + num_segments * segment_size
    --> stack_size_bytes = block_size - num_segments * segment_size
    stack_size_bytes = size_of::<*mut u8>() * num_segments
    --> num_segments = stack_size_bytes / size_of::<*mut u8>()

    stack_size_bytes = block_size - (stack_size_bytes / size_of::<*mut u8>()) * segment_size
    stack_size_bytes = block_size - (stack_size_bytes * segment_size) / size_of::<*mut u8>()
    stack_size_bytes + (stack_size_bytes * segment_size) / size_of::<*mut u8>() = block_size
    (size_of::<*mut u8>() * stack_size_bytes) / size_of::<*mut u8>() + (stack_size_bytes * segment_size) / size_of::<*mut u8>() = block_size
    (size_of::<*mut u8>() * stack_size_bytes + stack_size_bytes * segment_size) / size_of::<*mut u8>() = block_size
    stack_size_bytes * (size_of::<*mut u8>() + segment_size) / size_of::<*mut u8>() = block_size
    stack_size_bytes * (size_of::<*mut u8>() + segment_size) = block_size * size_of::<*mut u8>()
    stack_size_bytes = block_size * size_of::<*mut u8>() / (size_of::<*mut u8>() + segment_size)
    */

    let (num_segments, segment_size) = if segment_size > block_size {
        (
            1,
            block_size - size_of::<*mut u8>()
        )
    }
    else {
        let stack_size_bytes = (block_size * size_of::<*mut u8>()) / (size_of::<*mut u8>() + segment_size);
        let num_segments = stack_size_bytes / size_of::<*mut u8>();
        (
            num_segments,
            segment_size
        )
    };

    // Recalc stack size to keep alignment
    let stack_size_bytes = block_size - num_segments * segment_size;
    let payload_ptr = block_base_address.add(stack_size_bytes);

    if payload_ptr as usize % ALIGN != 0 {
        panic!("Bad alignment in payload -> {:#x} {}", payload_ptr as usize, stack_size_bytes);
    }

    // Convert to pointer of pointers
    let block_base_address = block_base_address as *mut *mut u8;

    // Create stack with pointers to all segments in the block
    for segment_index in 0..num_segments {
        let offset = segment_index * segment_size;
        *(block_base_address.add(segment_index)) = payload_ptr.add(offset);
    }

    // Create layout struct
    MemBlockLayout {
        stack_ptr: block_base_address,
        payload_ptr,
        segment_size,
        num_segments,
        used_segments: 0,
        block_size
    }
}