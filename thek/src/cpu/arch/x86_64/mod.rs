//! x86_64 CPU handling.

use x86_64::structures::{
    idt::{
        InterruptDescriptorTable, InterruptStackFrame
    }
};
use crate::sys::{
    KMutex
};
use crate::controllers::stdout::StdoutController;
use core::{
    write,
    fmt::Write
};

/// Init x86_64 essential interrupts.
pub fn init_ints() {
    let mut idt = unsafe { IDT.acquire() };
    // Set breakpoint interrupt handler
    idt.breakpoint.set_handler_fn(breakpoint_int_handler);
    // Set double fault interrupt handler
    idt.double_fault.set_handler_fn(double_fault_int_handler);
    // Load IDT
    unsafe {
        idt.load_unsafe();
    }
}

extern "x86-interrupt"
fn breakpoint_int_handler(stack_frame: InterruptStackFrame) {
    let mut con = StdoutController::default();
    write!(&mut con, "Breakpoint! = {:#?}\n", stack_frame).unwrap_or_default();
}

extern "x86-interrupt"
fn double_fault_int_handler(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
    panic!("DOUBLE FAULT = {} , {:#?}", error_code, stack_frame);
}

static mut IDT: KMutex<InterruptDescriptorTable> = KMutex::new(InterruptDescriptorTable::new());
