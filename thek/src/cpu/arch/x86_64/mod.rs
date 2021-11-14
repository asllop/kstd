//! x86_64 CPU handling.

use x86_64::{
    structures::{
        idt::{
            InterruptDescriptorTable, InterruptStackFrame
        },
        gdt::{
            GlobalDescriptorTable, Descriptor
        }
    },
    // instructions::{
    //     tables::load_tss
    // },
    registers::segmentation::{
        Segment, CS
    }
};
use crate::sys::{
    KMutex
};

/// Initialize ints, cpu structures, etc.
pub fn init() {
    init_gdt();
    init_ints();
}

// Init GDT.
fn init_gdt() {
    //TODO: create TSS
    let mut gdt = GDT.acquire();
    let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
    // let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
    // Load GDT
    unsafe {
        gdt.load_unsafe();
        CS::set_reg(code_selector);
        //load_tss(tss_selector);
    }
}

static GDT: KMutex<GlobalDescriptorTable> = KMutex::new(GlobalDescriptorTable::new());

// Init essential interrupts.
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

/// Halt the system.
pub fn halt() {
    unsafe {
        asm!("cli");
        asm!("hlt");
    }
}
