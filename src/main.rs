#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt;

#[macro_use]
mod console;
use console::*;

mod counter_future;
use counter_future::*;

use core::sync::atomic::Ordering;
use core::sync::atomic::AtomicI32;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_title("-- Rust Kernel Test --");
    print_count();
    println!("MT my_val {}", MT.my_val.load(Ordering::SeqCst));
    MT.my_val.store(100, Ordering::SeqCst);
    println!("After change, MT my_val {}", MT.my_val.load(Ordering::SeqCst));
    loop {}
}

// Experimental console usage
fn print_title(msg: &str) {
    let center = 40 - msg.len() / 2;

    let console = unsafe { CONSOLE_WRITER.console() };
    console << (center, 12, msg);

    let console = Console::new(ConsoleColor::Black, ConsoleColor::Yellow);
    &console << (center, 13, msg);
}

// Regular console usage
fn print_count() {
    for i in 0..10 {
        println!("Counter {}", i);
    }
    println!();
}