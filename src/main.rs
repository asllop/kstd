//!
//! # Memory Management
//! 
//! Pool of Segments: it's a variation of the bitmaps memory management system.
//! 
//! Flat model. Divide memory in buckets. Each bucket is a range of memory that contains memory segments of the same size.<br>
//! We have multiple buckets with different segment sizes to allocate memory with different requierements, because each Alloc returns exactly one segment.<br>
//! We have a fixed number of buckets ordered from smaller segment size to bigger.<br>
//! At start, we create a struct for each bucket, that contains:
//! 
//! - Stack. We put in the stack the starting address of each segment.
//! - Segment size (in byte).
//! - Bucket size (total number of segments).
//! - Counter with the current number of free segments.
//! 
//! Once an alloc happens, we check the size requested and we select the bucket with the closest segment size.
//! We pop an address from the stack and decrease the counter.
//! If no segments available in the bucklet, we try with the next bucket size, and so on.
//! 
//! When a free happens, we just push the segment address into the bucket (each segment has a header with a pointer to the bucket struct it belongs to), and increase the counter.
//! 
//! Advantages:
//! 
//! - Predictable and fast Alloc and Free operation times, O(1) complexity.
//! - No need for long mutex cycles that lock other tasks, only simple atomic PopAddress and PushAddress operation that are very short.
//! 
//! Disadvantages:
//! - Is not possible to guarantee contiguous segments when we alloc, and then we have less flexibility (resize operation is not feasible).
//! - More affected by fragmentation, more likely to get nothing from Alloc than other classic allocation methods (like linked lists).
//! 
//! Drawbacks can be mitigated by chosing convenient segment and bucket sizes.
//! 
//! # Device Model
//! 
//! We need a `Device` trait, that controls each device, with a missage passing interface (`DeviceMessage` trait and `DeviceResponse` trait) that is something like ioctl.
//! 
//! Then we need a `Driver` trait, that is the real interface the apps will use.
//! 
//! Drivers talk to Devices. Drivers are arch independant, Devices are arch dependant. One Driver can be used to talk to multiple Devices, for example: the ConsoleDriver could use the VideoDevice + KeyboardDevice or the SerialDevice.
//! 
//! We could have a pluggable interface for Inputs and Outputs, so we can configure drivers to work with any combination of devices, without having to be aware of which device it is. Maybe a series of traits: `InputFlow`, `OutputFlow`, `BidiFlow`.<br>
//! We could use it to create complex flow chains: a ConsoleDriver that reads input from a KeyboardDevice, but sends the output to a TcpDriver that in turn sends it to the SlipDriver that sends it to the SerialDevice.
//! 


#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt;

/*
#[macro_use]
mod console;
use console::*;

mod counter_future;
use counter_future::*;

use core::sync::atomic::Ordering;
use core::sync::atomic::AtomicI32;
*/

mod devices;
use devices::{
    InputFlow, OutputFlow,
    console::{
        ConCmd, ConsoleDevice, CON_DEVICE, AnsiColor
    }
};

mod controllers;
use controllers::{
    console:: {
        ScreenConsole, ConsoleController
    }
};

mod sys;
use sys::{
    KMutex, KLock
};

/*
static TEST : KMutex<TestStruct> = KMutex::new(TestStruct { count: 0, buf: [0;32] });

struct TestStruct {
    count: usize,
    buf: [u8 ; 32]
}
*/

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    CON_DEVICE.reset();
    let mut con = ScreenConsole::new(AnsiColor::BrightWhite, AnsiColor::Red);
    con.set_xy(0, 0);
    w_print!(con, "### Kernel {} ###", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_one();

    {
        let mut con = ScreenConsole::default();
        w_print!(con, "\n\n\n\nHolaaa!!");
    }

    //_fail();

    //print_two();
    /*
    print_title("-- Rust Kernel Test --");
    print_count();
    println!("MT my_val {}", MT.my_val.load(Ordering::SeqCst));
    MT.my_val.store(100, Ordering::SeqCst);
    println!("After change, MT my_val {}", MT.my_val.load(Ordering::SeqCst));

    CON_DEVICE.lock().write_cmd(
        ConCmd::Print(10, 24, AnsiColor::BrightRed, AnsiColor::BrightCyan),
        "Final thing!".as_bytes()
    ).unwrap_or_default();

    CON_DEVICE.lock().write_cmd(
        ConCmd::Print(30, 24, AnsiColor::BrightRed, AnsiColor::BrightCyan),
        "One more thing".as_bytes()
    ).unwrap_or_default();

    println!("TEST lock count = {}", TEST.lock().count);
*/
    loop {}
}

fn print_one() {
    let x = 101;
    println!("---->");
    print_two();
    println!("\nNumber 1 = {}", x);
}

fn print_two() {
    let x = 202;
    println!("\n\n\nNumber 2 = {}", x);
}

fn _fail() {
    let a : Option<i32> = None;
    //panic
    a.unwrap();
}

/*
// Experimental console usage
fn print_title(msg: &str) {
    let center = 40 - msg.len() / 2;

    let console = unsafe { CONSOLE_WRITER.console() };
    console << (center, 11, msg);

    let console = Console::new(ColorScheme::default());
    &console << (center, 13, msg);

    let mut console = Console::default();
    console[ConIndex::Cha(75, 20)] = b'A';
    console[ConIndex::Col(75, 20)] = 0x0Au8;
    console[ConIndex::Cha(76, 20)] = b'B';
    console[ConIndex::Col(76, 20)] = 0x0Bu8;
    console[ConIndex::Cha(77, 20)] = b'C';
    console[ConIndex::Col(77, 20)] = 0x0Cu8;
}

// Regular console usage
fn print_count() {
    for i in 0..10 {
        println!("Counter {}", i);
    }
    println!();
}
*/