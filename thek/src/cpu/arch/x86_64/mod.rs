//! x86_64 CPU handling.

use x86_64::{
    VirtAddr,
    structures::{
        idt::{
            InterruptDescriptorTable, InterruptStackFrame
        },
        gdt::{
            GlobalDescriptorTable, Descriptor
        }
    },
    instructions::{
        //tables::load_tss
        interrupts::are_enabled
    },
    registers::segmentation::{
        Segment, CS
    }
};
use pic8259::ChainedPics;
use crate::sys::KMutex;

/// Initialize ints, cpu structures, etc.
pub fn init_arch() {
    init_gdt();
    init_idt();
    init_pic();
    setup_timer();
}

/// Enable interrupts, timers, etc.
pub fn start_arch() {
    enable_ints();
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
    unsafe {
        idt[PicInt::Timer as usize].set_handler_addr(VirtAddr::new(timer_int_handler as u64));
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

#[derive(Clone)]
#[repr(C)]
/// Stored register on every interrupt.
pub struct StackFrame {
    // Registers pushed by the ISR
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rbp: u64,
    // Interrupt Stack Frame
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64
}

#[inline(never)]
extern "C"
fn timer_isr(stack_frame: &StackFrame) {
    let th = TIMER_HANDLER.acquire();
    (*th)(stack_frame);
    unsafe {
        PICS.acquire().notify_end_of_interrupt(PicInt::Timer as u8);
    }
}

//TODO: save stack frame(5 registers) + scratch registers(15 registers) = 20 registers * 8 bytes/register = 160 bytes
#[naked]
unsafe extern "C" fn timer_int_handler() {
    asm!("
        # Clear ints and store all registers (the ones not already stored in the interrupt stack frame).
        cli
        push rbp
        push r15
        push r14
        push r13
        push r12
        push r11
        push r10
        push r9
        push r8
        push rsi
        push rdi
        push rdx
        push rcx
        push rbx
        push rax

        # Call the actual ISR, passing as argument (RDI) a pointer to the saved registers (RSP)
        mov rdi, rsp
        call {}

        # Recover registers, set interrupts and return.
        pop rax
        pop rbx
        pop rcx
        pop rdx
        pop rdi
        pop rsi
        pop r8
        pop r9
        pop r10
        pop r11
        pop r12
        pop r13
        pop r14
        pop r15
        pop rbp
        sti
        iretq
    ", sym timer_isr, options(noreturn));
}

// Frequency divisor (1 millisecond resolution).
const FREQ_DIVISOR: u16 = 1200;

/// Timer frequency in Hz.
pub const TIMER_FREQ_HZ: f64 = 1193181.6666 / FREQ_DIVISOR as f64;

fn setup_timer() {
    // Set: Channel 0, lobyte/hibyte, Mode 2, Binary
    let cmd: u8 = 0b_00_11_010_0;
    outb(0x43, cmd);
    // Set freq divisor (lobyte, hibyte)
    let divisor: u16 = FREQ_DIVISOR;
    outb(0x40, (divisor & 0xFF) as u8);
    outb(0x40, ((divisor >> 8) & 0xFF) as u8);
}

/// Set a function to be executed on each timer interrupt.
pub fn set_timer_handler(func: fn(&StackFrame)) {
    let mut th = TIMER_HANDLER.acquire();
    *th = func;
}

static TIMER_HANDLER: KMutex<fn(&StackFrame)> = KMutex::new(|_| {});

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

#[inline]
/// Halt the system.
pub fn halt() {
    unsafe {
        asm!("
            cli
            hlt
        ");
    }
}

#[inline]
/// Disable interrupts.
pub fn disable_ints() {
    unsafe {
        asm!("cli");
    }
}

#[inline]
/// Enable interrupts.
pub fn enable_ints() {
    unsafe {
        asm!("sti");
    }
}

#[inline]
/// Check if interrupts are enabled.
pub fn check_ints() -> bool {
    are_enabled()
}

/* TODO:
Trick to check stack integrity and avoid overflow in No Mem Protection systems:
- Create a stack that is bigger than the required, let's say N bytes more. We call this extra bytes the sanger zone.
- Count the time needed by the current CPU to push N bytes to stack (using a recursion func call), and use this as the switching period.
- On everty task switch, check the stack, if it's in the danger zone, abort the task.
*/
