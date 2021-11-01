//! Dynamic memory layout.

use crate::sys::KError;

/// Maximum number of allowed memory blocks.
pub const MAX_NUM_BLOCKS: usize = 5;

/// Memory blocks set struct.
#[repr(C)]
pub struct MemBlockSet {
    /// Pointer to stack of segments.
    pub block_layouts: [MemBlockLayout; MAX_NUM_BLOCKS],
    /// Number of valid blocks in the array.
    pub num_blocks: usize
}

impl MemBlockSet {
    /// Get reference to block at specified index.
    pub fn block_at(&mut self, index: usize) -> Option<&mut MemBlockLayout> {
        if index < self.len() {
            Some(&mut self.block_layouts[index])
        }
        else {
            None
        }
    }

    /// Find a block to allocate a buffer of the requested size.
    pub fn find_block(&mut self, buf_size: usize) -> Option<&mut MemBlockLayout> {
        for i in 0..self.len() {
            if self.block_layouts[i].segment_size >= buf_size &&
               self.block_layouts[i].used_segments < self.block_layouts[i].num_segments {
                   return self.block_at(i);
            }
        }
        None
    }

    /// Find the block that owns the provided segment.
    pub unsafe fn owns_segment(&mut self, segment_ptr: *mut u8) -> Option<&mut MemBlockLayout> {
        for i in 0..self.len() {
            let payload_initial_ptr = self.block_layouts[i].payload_ptr;
            let payload_size = self.block_layouts[i].segment_size * self.block_layouts[i].num_segments;
            let payload_final_ptr = payload_initial_ptr.add(payload_size);

            let distance = payload_final_ptr.offset_from(segment_ptr);
            if distance > 0 && distance <= payload_size as isize {
                return self.block_at(i);
            }
        }
        None
    }

    /// Number of memory blocks.
    pub fn len(&self) -> usize {
        self.num_blocks
    }
}

/// Memory layout struct
#[repr(C)]
pub struct MemBlockLayout {
    /// Pointe to stack of segments
    pub stack_ptr: *mut *mut u8,
    /// Pointer to usable memory
    pub payload_ptr: *mut u8,
    /// Segment size in bytes
    pub segment_size: usize,
    /// Total number of segments in the block
    pub num_segments: usize,
    /// Number of segments currently in use
    pub used_segments: usize,
    /// Block size
    pub block_size: usize
}

impl MemBlockLayout {
    pub fn empty() -> Self {
        Self {
            stack_ptr: 0 as *mut *mut u8,
            payload_ptr: 0 as *mut u8,
            segment_size: 0,
            num_segments: 0,
            used_segments: 0,
            block_size: 0
        }
    }

    /// Pop address from stack.
    pub unsafe fn pop_address(&mut self) -> Option<*mut u8> {
        if self.used_segments < self.num_segments {
            // Calculate current stack top pointer
            let stack_top = self.stack_ptr.add(self.used_segments);
            // Get value in the top of stack
            let ptr = *stack_top;
            // Set zero
            *stack_top = 0 as *mut u8;
            // Increment number of used segments
            self.used_segments += 1;
            Some(ptr)
        }
        else {
            None
        }
    }

    /// Push address to stack
    pub unsafe fn push_address(&mut self, ptr: *mut u8) -> Result<(), KError> {
        if self.used_segments > 0 {
            // Decrement number of used segments
            self.used_segments -= 1;
            // Calculate current stack top pointer
            let stack_top = self.stack_ptr.add(self.used_segments);
            // Set ptr
            *stack_top = ptr;
            Ok(())
        }
        else {
            panic!("Trying to push segment pointer on a full stack");
        }
    }
}