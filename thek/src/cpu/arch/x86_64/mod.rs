//! x86_64 CPU handling.

use x86_64::structures::{
    idt::{
        InterruptDescriptorTable, InterruptStackFrame
    }
};
use crate::sys::{
    KMutex
};

/// Init x86_64 essential interrupts.
pub fn init_ints() {
    let mut idt = IDT.acquire();
    // Set double fault interrupt handler
    idt.double_fault.set_handler_fn(double_fault_int_handler);
    // Load IDT
    unsafe {
        idt.load_unsafe();
    }
}

extern "x86-interrupt"
fn double_fault_int_handler(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
    panic!("DOUBLE FAULT = {} , {:#?}", error_code, stack_frame);
}

static IDT: KMutex<InterruptDescriptorTable> = KMutex::new(InterruptDescriptorTable::new());
