use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use alloc::vec::Vec;
use crate::{
    cpu::{
        arch::{
            StackFrame, set_timer_handler
        }
    },
    mem::KBox,
    sys::KMutex
};

pub struct Task {
    // name + name_len = 16 bytes, this way we avoid padding
    name: [u8; 15],
    name_len: u8,
    pub func: fn(),
    pub stack: KBox
}

impl Task {
    pub fn new(name_str: &str, stack_size: usize, func: fn()) -> Result<Self, ()> {
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

/// Init tasks module.
pub fn init_task() {
    set_timer_handler(internal_timer_handler);
}

fn internal_timer_handler(stack_frame: &StackFrame) {
    super::tick();

    if TASK_SWITCHING.load(Ordering::SeqCst) {
        let tasks_vec = TASKS.acquire();
        //TODO: stack_frame is the current stack pointer, store it in the current Task
        // Calculate next task index
        let last_index = TASK_INDEX.fetch_add(1, Ordering::SeqCst);
        let index = if last_index == tasks_vec.len() - 1 {
            TASK_INDEX.store(0, Ordering::SeqCst);
            0
        }
        else {
            last_index + 1
        };
        //TODO: get task stack pointer of next task and use it as current stack frame: PROBLEM, we are not the ISR, we have other things in the stack.
    }
}

/// Enable task scheduling.
/// 
/// Returns previous value.
pub fn enable_scheduling() -> bool {
    TASK_SWITCHING.swap(true, Ordering::SeqCst)
}

/// Disable task scheduling.
/// 
/// Returns previous value.
pub fn disable_scheduling() -> bool {
    TASK_SWITCHING.swap(false, Ordering::SeqCst)
}

// Task switching flag.
static TASK_SWITCHING: AtomicBool = AtomicBool::new(false);
// Index of current task.
static TASK_INDEX: AtomicUsize = AtomicUsize::new(usize::MAX);
// List of tasks.
static TASKS: KMutex<Vec<Task>> = KMutex::new(Vec::new());

// /// Get a lock and index to current task.
// pub fn current() -> (KLock<'static, Vec<Task>>, usize) {
//     let vec = TASKS.acquire();
//     let index = TASK_INDEX.load(Ordering::Relaxed);
//     (vec, index)
// }

const DEFAULT_STACK_SIZE: usize = 4*1024;

/// Start a new task.
pub fn start(name: &str, stack_size: Option<usize>, func: fn()) {
    let prev_val = TASK_SWITCHING.swap(false, Ordering::SeqCst);
    if let Ok(task) = Task::new(name, stack_size.unwrap_or(DEFAULT_STACK_SIZE), func) {
        let mut tasks_vec = TASKS.acquire();
        tasks_vec.push(task);
    }
    TASK_SWITCHING.store(prev_val, Ordering::SeqCst);
}
