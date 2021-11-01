#![no_main]
#![no_std]

use thek::{
    print, println,
    controllers::console::{
        ansi::AnsiColor,
        DefaultConsoleController, ConsoleController
    },
    mem::{
        arch::raw_mem,
        layout::{
            MemBlockSet, //MemBlockLayout
        }
    }
};

use std::prelude::v1::*;

use std::fmt::Write;

use core::mem::size_of;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    thek::mem::init::setup_mem(&[
        (4*1024, 50),       // 50% of mem in segments of 4KB
        (256*1024, 25),     // 25% of mem in segments of 256KB
        (usize::MAX, 25)    // Remaining 25% in one single segment
    ]);
    main();
    loop {}
}

/*
TODO: create tests for mem
- Check used mem after every alloc/dealloc
- Check if segments are correctly selected depending on requiered size.
- Check that blocks are not overlaping.
*/

fn main() {

    {
        let mut con = DefaultConsoleController::new(AnsiColor::BrightWhite, AnsiColor::BrightBlue);
        con.set_xy(33, 0).unwrap_or_default();
        writeln!(&mut con, " <<< TheK >>>");
    }

    let block_set = unsafe {
        let (ptr, _) = raw_mem();
        &mut*(ptr as *mut MemBlockSet)
    };

    for i in 0..block_set.len() {
        let blay = block_set.block_at(i).unwrap();
        println!("({}) - {:#x} , seg={} , blck={} , stck={}, pyld={}",
            blay.num_segments,
            blay.payload_ptr as usize,
            blay.segment_size,
            blay.block_size,
            blay.num_segments * size_of::<*mut u8>(),
            blay.num_segments * blay.segment_size
        );
/*
        if blay.num_segments > 0 {
            let addr1 = unsafe { blay.pop_address().unwrap() };
            println!("   {:#x}", addr1 as usize);
        }
        if blay.num_segments > 1 {
            let addr2 = unsafe { blay.pop_address().unwrap() };
            println!("   {:#x}", addr2 as usize);
        }
        if blay.num_segments > 2 {
            let addr3 = unsafe { blay.pop_address().unwrap() };
            println!("   {:#x}", addr3 as usize);
        }
*/
    }

    let v = vec!(1,2,3,4,5,6,7,8,9);
    for (i, x) in v.iter().enumerate() {
        print!("[{}] = {}, ", i, x);
    }
    println!();

    let s = String::from("Aix`o mola molt! e's guay!");
    println!("string = {}", s);

    let b = Box::new(1200);
    println!("box = {}", b);

    let v2 = vec!(1,2,3,4,5,6,7,8,9);
    for (i, x) in v2.iter().enumerate() {
        print!("[{}] = {}, ", i, x);
    }
    println!();

    print_count(1);

    _fail_unwrap();

    //_fail_oom();
}

fn print_count(n: i32) {
    for i in 0..n {
        println!("Counter {}", i);
    }
}

fn _fail_unwrap() {
    let a : Option<i32> = None;
    //panic
    a.unwrap();
}

fn _fail_oom() {
    // Crash, out of memory!
    let mut v = Vec::new();
    loop {
        v.push(String::from("Nova cadena"));
    }
}