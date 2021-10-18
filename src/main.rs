#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt;

#[macro_use]
mod console;
use console::*;

mod counter_future;
use counter_future::*;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_title("-- Rust Kernel Test --");
    print_count();
    loop {}
}

fn print_title(msg: &str) {
    let center = 40 - msg.len() / 2;
    &CONSOLE << (center, 12, msg);
}

fn print_count() {
    for i in 0..30 {
        print!("Counter {}\n", i);
    }
}