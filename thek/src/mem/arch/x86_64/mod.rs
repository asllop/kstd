//! Memory infrastructure for x86_64.

/// Return pointer, size and alignment.
pub unsafe fn raw_mem() -> (*mut u8, usize) {
    (RAW_MEMORY.as_mut_ptr(), MEM_SIZE)
}

/// Memory alignment
pub const ALIGN : usize = 4;

/// Simulated raw memory (until we access actual raw mem)
const MEM_SIZE : usize = 10*1024*1024;
static mut RAW_MEMORY : [u8; MEM_SIZE] = [0; MEM_SIZE];