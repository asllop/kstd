#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod console;
use console::*;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_title("-- Rust Kernel Test --");
    loop {}
}

fn print_title(msg: &str) {
    let center = 40 - msg.len() / 2;
    &CONSOLE << (center, 12, msg);
}