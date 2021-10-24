//! Dynamic memory layout.

use crate::sys::KError;

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
    pub used_segments: usize
}

impl MemBlockLayout {
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