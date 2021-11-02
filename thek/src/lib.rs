//! # Introduction
//! 
//! `TheK` is a unikernel specially designed to offer support to Rust [`std`](https://doc.rust-lang.org/std/).
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
//! - Is not possible to guarantee contiguous segments when we alloc, and then we have less flexibility.
//! - More affected by fragmentation, more likely to get nothing from Alloc than other classic allocation methods (like linked lists).
//! 
//! Drawbacks can be mitigated by chosing convenient segment and bucket sizes.
//! 
//! # Drivers
//! 
//! Drivers are splitted into 2 parts:
//! 
//! - **Devices** access hardware directly. They usually only offer very low level features, directly support by the underlying hardware. They implement API traits for interaction with the external world, like [`PlotTextDevice`][`devices::plot::text::PlotTextDevice`].
//! - **Controllers** are arch independant and they use devices as an abstraction layer to control the hardware. They implement traits to offer a standard interface to users, like [`ConsoleController`].
//! 
//! Users should generally access controllers, because they offer a higher abstraction level and more features. Only use devices directly whenever you have a very specific and low level requirement.
//! 
//! # Next steps:
//! 
//! - Explore OS dependencies to build std.
//! - Implement SMP support.
//! - Implement multithreading.
//! - Implement async (optional).
//! - Explore UEFI support of keyboard input, filesystem (others?).
//! - Implement a PCI driver and...
//! - Implement USB driver (based on LibUSB).

/*
Crates tp simplify x86 low level handling:

https://docs.rs/x86_64/
https://docs.rs/x86_interrupts
https://docs.rs/x86/
*/

#![no_std]
#![feature(asm)]
#![feature(alloc_error_handler)] 

pub mod arch;

pub mod devices;

pub mod controllers;

pub mod sys;

pub mod mem;

use controllers::plot::text::{
    ansi::AnsiColor,
    PlotTextController
};

use devices::{
    Device,
    plot::text::ScreenTextDevice
};

use core::{
    panic::PanicInfo,
    fmt::Write
};

/// Default console controller
pub type DefaultConsoleController<'a> = PlotTextController<'a, ScreenTextDevice>;

/// Panic handler.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ScreenTextDevice::reset_lock();
    let mut con = PlotTextController::<ScreenTextDevice>::new(
        AnsiColor::BrightWhite,
        AnsiColor::Red
    );
    con.set_xy(0, 0).unwrap_or_default();
    write!(&mut con, "### Kernel {} ###", info).unwrap_or_default();
    loop {
        arch::halt();
    }
}