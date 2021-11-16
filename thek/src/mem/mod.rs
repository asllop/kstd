//! Memory management.

pub mod layout;

mod globalloc;

pub mod init;

pub mod arch;

/// Initialize a default memory schema optimized for small allocations.
pub fn init_small_schema() {
    init::setup_mem(&[
        (256, 80),              // 80% of mem in segments of 256Bytes
        (1024, 10),             // 10% of mem in segments of 1K
        (usize::MAX - 1, 5),    // Remaining 10% in two segments
        (usize::MAX, 5)
    ]);
}

/// Initialize a default memory schema optimized for big allocations.
pub fn init_big_schema() {
    init::setup_mem(&[
        (4*1024, 10),           // 10% of mem in segments of 4K
        (128*1024, 80),         // 80% of mem in segments of 128K
        (usize::MAX - 1, 5),    // Remaining 10% in two segments
        (usize::MAX, 5)
    ]);
}
