//! Task scheduling.

use core::sync::atomic::{
    AtomicUsize, Ordering
};
use alloc::vec::Vec;
use crate::{
    cpu::{
        time, arch::StackFrame
    },
    sys::{
        KMutex, KLock
    },
    mem::KBox
};

pub struct Task {
    // name + name_len = 16 bytes, this way we avoid padding
    name: [u8; 15],
    name_len: u8,
    pub func: fn(),
    pub stack: KBox,
    pub state: Option<StackFrame>
}

impl Task {
    pub fn new(name_str: &str, func: fn(), stack_size: usize) -> Result<Self, ()> {
        if name_str.bytes().len() > 15 {
            return Err(());
        }
        let mut name: [u8; 15] = [0; 15];
        let mut name_len: u8 = 0;
        for (i, ch) in name_str.bytes().enumerate() {
            name[i] = ch;
            name_len += 1;
        }
        Ok(
            Self {
                name,
                name_len,
                func,
                stack: KBox::new(stack_size)?,
                state: None
            }
        )
    }

    pub fn name_str(&self) -> &str {
        unsafe {
            let slice = core::slice::from_raw_parts(
                &self.name as *const u8, 
                self.name_len as usize
            );
            core::str::from_utf8_unchecked(slice)
        }
    }
}

pub fn init_task() {
    if !time::register_handler(task_timer_handler, 0) {
        panic!("Couldn't create task scheduler");
    }
}

// Index of current task.
static TASK_INDEX: AtomicUsize = AtomicUsize::new(usize::MAX);
// List of tasks.
static TASKS: KMutex<Vec<Task>> = KMutex::new(Vec::new());

fn task_timer_handler(stack_frame: &StackFrame) {
    let _state = stack_frame.clone();
    //TODO: swap task
    // - Disable ints
    // - Get current task from TASKS[TASK_INDEX].
    // - Get interrupt stack frame and registers and save them all in the Task struct. Put back to TASKS array.
    // - Increment TASK_INDEX. If out of bounds, go to 0.
    // - Get next task from TASKS[TASK_INDEX]
    // - Put interrupt stack frame from Task struct to stack.
    // - Enable ints
    // - Restore registers and IRETQ.
}

/// Get a lock and index to current task.
pub fn current() -> (KLock<'static, Vec<Task>>, usize) {
    let vec = TASKS.acquire();
    let index = TASK_INDEX.load(Ordering::Relaxed);
    (vec, index)
}

pub fn start(_name: &str, _func: fn()) {
    // TODO: create a new task and pass control to it
    // - Alloc KBox for the stack
    // - Create a Task struct with name, stack, func pointer, and empty state.
    // - Create a new entry in TASKS
}
