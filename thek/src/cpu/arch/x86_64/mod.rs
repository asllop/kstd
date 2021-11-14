//! x86_64 CPU handling.

use x86_64::structures::{
    idt::{
        InterruptDescriptorTable, InterruptStackFrame
    }
};
use crate::sys::{
    KMutex
};

/// Initialize ints, memory segments, etc.
pub fn init() {
    init_ints();
}

/// Init x86_64 essential interrupts.
fn init_ints() {
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

//TODO: set timer imterrupt and create a task switcher


/// Input byte from port
pub fn inb(port: u16) -> u8 {
    let r: u8;
    unsafe {
        asm!("in al, dx", out("al") r, in("dx") port);
    }
    r
}

/// Output byte to port
pub fn outb(port: u16, data: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") data);
    }
}

/// Halt
pub fn halt() {
    unsafe {
        asm!("cli");
        asm!("hlt");
    }
}
