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
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    // Load IDT
    unsafe {
        idt.load_unsafe();
    }
}

extern "x86-interrupt"
fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    let mut con = StdoutController::default();
    write!(&mut con, "Breakpoint! = {:#?}\n", stack_frame).unwrap_or_default();
}

static mut IDT: KMutex<InterruptDescriptorTable> = KMutex::new(InterruptDescriptorTable::new());

//TEST
pub fn int_3() {
    x86_64::instructions::interrupts::int3();
}
