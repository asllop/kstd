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
use pic8259::ChainedPics;
use crate::sys::KMutex;

/// Initialize ints, cpu structures, etc.
pub fn init() {
    init_gdt();
    init_idt();
    init_pic();
    setup_timer();
}

/// Enable interrupts, timers, etc.
pub fn start() {
    unsafe { asm!("sti"); }
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
fn init_idt() {
    let mut idt = IDT.acquire();
    // Set double fault interrupt handler
    //TODO: set a different stack for this handler
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

#[repr(u8)]
enum PicInt {
    Timer = PIC_1_OFFSET + 0,
}

fn init_pic() {
    unsafe {
        PICS.acquire().initialize();
    }
    let mut idt = IDT.acquire();
    idt[PicInt::Timer as usize].set_handler_fn(timer_int_handler);
    unsafe {
        idt.load_unsafe();
    }
}

const PIC_1_OFFSET: u8 = 32;
const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

static PICS: KMutex<ChainedPics> = KMutex::new(
    unsafe {
        ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)
    }
);

extern "x86-interrupt"
fn timer_int_handler(_stack_frame: InterruptStackFrame) {
    //thek_dbg!("Timer!");
    unsafe {
        PICS.acquire().notify_end_of_interrupt(PicInt::Timer as u8);
    }
}

fn setup_timer() {
    // Set: Channel 0, lobyte/hibyte, Mode 2, Binary
    let cmd: u8 = 0b_00_11_010_0;
    outb(0x43, cmd);
    // Set freq divisor (lobyte, hibyte)
    let divisor: u16 = 5000;
    outb(0x40, (divisor & 0xFF) as u8);
    outb(0x40, ((divisor >> 8) & 0xFF) as u8);
}

#[inline]
/// Input byte from port
pub fn inb(port: u16) -> u8 {
    let r: u8;
    unsafe {
        asm!("in al, dx", out("al") r, in("dx") port);
    }
    r
}

#[inline]
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

/* TODO:
Trick to check stack integrity and avoid overflow in No Mem Protection systems:
- Create a stack that is bigger than the required, let's say N bytes more. We call this extra bytes the sanger zone.
- Count the time needed by the current CPU to push N bytes to stack (using a recursion func call), and use this as the switching period.
- On everty task switch, check the stack, if it's in the danger zone, abort the task.
*/
