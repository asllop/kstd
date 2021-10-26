#![no_main]
//#![feature(restricted_std)]
#![no_std]

use thek::{
    print, println, w_print,
    controllers::console::{
        ansi::AnsiColor,
        ScreenConsoleController, ConsoleController
    },
    mem::{
        arch::raw_mem,
        layout::{
            MemBlockSet, MemBlockLayout
        }
    }
};

extern crate alloc;

use alloc::{
    vec,
    vec::Vec,
    string::String,
    boxed::Box
};

use core::mem::size_of;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    thek::mem::init::setup_mem(&[
        (4*1024, 50),
        (256*1024, 25),
        (usize::MAX, 25)
    ]);
    main();
    loop {}
}

fn main() {

    println!("MemBlockSet size {}", size_of::<MemBlockSet>());

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

/*
    print!("\nHola");
    print_one();
    print_two();
    print_count(3);
    print!("Adeu!");
    println!();

    {
        let mut con = ScreenConsoleController::new(AnsiColor::BrightWhite, AnsiColor::BrightBlue);
        con.set_xy(34, 12).unwrap_or_default();
        w_print!(con, "<<< The K >>>");
    }
*/
    //_fail_unwrap();
    //_fail_index();
}

fn print_one() {
    let x = 101;
    println!("---->");
    println!("\nNumber 1 = {}", x);
}

fn print_two() {
    let x = 202;
    println!("\nNumber 2 = {}", x);
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

fn _fail_index() {
    // Read data from raw memory to avoid rust detecting index out of bounds at compile time
    let i = unsafe {
        *(0xB8000 as *mut u8)
    };

    let a = [1,2,3];
    let _x = a[i as usize];
}

fn _fail_oom() {
    // Crash, out of memory!
    let mut v = Vec::new();
    loop {
        v.push(String::from("Nova cadena"));
    }
}