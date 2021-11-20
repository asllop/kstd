use core::{
    ptr,
    mem,
    sync::atomic::{
        AtomicBool, AtomicUsize, Ordering
    }
};
use alloc::vec::Vec;
use crate::{
    cpu::arch::{
        StackFrame, set_timer_handler
    },
    mem::KBox,
    sys::KMutex,
    thek_dbg
};

#[derive(Debug)]
/// Task representation.
pub struct Task {
    // name + name_len = 16 bytes, this way we avoid padding
    name: [u8; 15],
    name_len: u8,
    func: fn(),
    stack_pointer: *mut u8,
    stack: KBox
}

impl Task {
    /// Create new task.
    pub fn new(name_str: &str, stack_size: usize, func: fn()) -> Result<Self, ()> {
        if name_str.bytes().len() < 16 {
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
                    stack_pointer: ptr::null_mut(),
                    stack: KBox::new(stack_size)?,
                }
            )
        }
        else {
            Err(())
        }
    }
    
    /// Get task name.
    pub fn name(&self) -> &str {
        unsafe {
            let slice = core::slice::from_raw_parts(
                &self.name as *const u8, 
                self.name_len as usize
            );
            core::str::from_utf8_unchecked(slice)
        }
    }
}

/// Init task module.
pub fn init_task() {
    set_timer_handler(internal_timer_handler);
}

fn internal_timer_handler(stack_frame: &StackFrame) -> *mut u8 {
    super::tick();

    if TASK_SWITCHING.load(Ordering::SeqCst) {
        let mut tasks_vec = TASKS.acquire();

        //TODO: store stack pointer in the current task
        let current_index = TASK_INDEX.load(Ordering::SeqCst);
        if current_index == usize::MAX {
            // TODO: first task swtich
            if tasks_vec.len() > 0 {

            }
            else {
                panic!("Tasks vector is empty");
            }
        }

        // Calculate next task index
        let last_index = TASK_INDEX.fetch_add(1, Ordering::SeqCst);
        let index = if last_index == tasks_vec.len() - 1 {
            TASK_INDEX.store(0, Ordering::SeqCst);
            0
        }
        else {
            last_index.overflowing_add(1).0
        };
        thek_dbg!(index);
        let task = &mut tasks_vec[index];
        thek_dbg!(&task);
        // Task not initialized yet
        if task.stack_pointer == ptr::null_mut() {
            let new_stack_pointer = unsafe { task.stack.top().sub(mem::size_of::<StackFrame>()) };
            // Add a margin to see if the stack if underflowing
            let new_stack_pointer = unsafe { new_stack_pointer.sub(8*16) };

            let new_stack_frame = stack_frame.new_task_stack(task.func, new_stack_pointer);
            thek_dbg!(&new_stack_frame);
            // Copy the new stack frame into the new stack
            unsafe {
                let src = mem::transmute::<&StackFrame, *const u8>(&new_stack_frame);
                ptr::copy(src, new_stack_pointer, mem::size_of::<StackFrame>());
            }
            task.stack_pointer = new_stack_pointer;
            thek_dbg!(&task);
            
            //TODO: we have to push a return address for the function, so it ends correctly

            // CRASH:
            /*
            Está petant a la primera instrucció de main, que és un push.
            Potser el SS per defecte no pot accedir a l'adreça de memòria que té la nova pila? Cal implementar una pagiunació com deu mana, no la merda que ens dona el bootloader.

            Primeres instruccions de la funció main:

_ZN4kstd4main17h4eb139e0b81c23bdE:
.Lfunc_begin231:
	.loc	46 51 0
	.cfi_startproc
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%rbx
	.cfi_def_cfa_offset 32
	subq	$4096, %rsp
    
    ...
    ...
            */
            
        }

        task.stack_pointer
    }
    else {
        ptr::null_mut()
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

const DEFAULT_STACK_SIZE: usize = 8*1024;

/// Start a new task.
pub fn start(name: &str, stack_size: Option<usize>, func: fn()) {
    let prev_val = TASK_SWITCHING.swap(false, Ordering::SeqCst);
    if let Ok(task) = Task::new(name, stack_size.unwrap_or(DEFAULT_STACK_SIZE), func) {
        let mut tasks_vec = TASKS.acquire();
        tasks_vec.push(task);
    }
    TASK_SWITCHING.store(prev_val, Ordering::SeqCst);
}
