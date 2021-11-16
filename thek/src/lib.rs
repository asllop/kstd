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
//! - **Devices** access hardware directly. They usually only offer very low level features, directly support by the underlying hardware. They implement API traits for interaction with the external world.
//! - **Controllers** are arch independant and they use devices as an abstraction layer to control the hardware. Each one can work with one type of devices.
//! 
//! Users should generally access controllers, because they offer a higher abstraction level and more features. Only use devices directly whenever you have a very specific and low level requirement.
//! 
//! # Choosing The Right Memory Schema
//! 
//! TODO
//! 
//! # Real Time Applications
//! 
//! Designing RT apps requieres some care, here we are going to mention some of the common pitfalls and alternatives to avoid them.
//! 
//! ## Allocating Memory
//! 
//! TODO
//! 
//! ### Hash Maps
//! 
//! TODO
//! 
//! # Next steps:
//! 
//! - Implement SMP support.
//! - Implement multithreading.
//! - Implement async (optional).
//! - Explore UEFI support of keyboard input, filesystem (others?).
//! - Implement a PCI driver and...
//! - Implement USB driver (based on LibUSB).

#![no_std]
#![feature(asm)]
#![feature(alloc_error_handler)]
#![cfg_attr(
    feature = "pc64",
    feature(abi_x86_interrupt)
)]

pub mod devices;

pub mod controllers;

pub mod sys;

pub mod mem;

pub mod cpu;

pub mod task;

//#[macro_use]
extern crate alloc;
use alloc::borrow::ToOwned;

use controllers::text::{
    TextController
};

use devices::{
    Device,
    text::{
        ansi::AnsiColor
    }
};

use core::{
    panic::PanicInfo,
    fmt::Write
};

#[macro_export]
macro_rules! thek_dbg {
    () => {
        let mut con = crate::controllers::port::PortController::default();
        core::fmt::write(&mut con, core::format_args!("[{}:{}]\n", file!(), line!())).unwrap_or(())
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                let mut con = crate::controllers::port::PortController::default();
                core::fmt::write(
                    &mut con,
                    core::format_args!(
                        "[{}:{}] {} = {:#?}\n",
                        file!(), line!(), stringify!($val), &tmp
                    )
                ).unwrap_or(());
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::thek_dbg!($val)),+,)
    };
}

/// Panic handler.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cpu::disable_ints();
    let dev_id = "CON1";
    if let Some(device) = devices::get_text_device(dev_id) {
        if let Device::Text(txt_dev) = device {
            // Reset mutex, just in case we panicked while still holding a lock.
            txt_dev.reset();
        }
        let mut con = TextController::new(
            AnsiColor::BrightWhite,
            AnsiColor::Red,
            dev_id.to_owned()
        ).unwrap();
        con.set_xy(0, 0).unwrap_or_default();
        write!(&mut con, "### Kernel {} ###", info).unwrap_or_default();
    }

    loop {
        cpu::halt();
    }
}
