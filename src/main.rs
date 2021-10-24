#![no_main]
//#![feature(restricted_std)]
#![no_std]

use thek::{
    print, println, w_print,
    controllers::console::{
        ansi::AnsiColor,
        ScreenConsoleController, ConsoleController
    }
};

extern crate alloc;

use alloc::{
    vec,
    vec::Vec,
    string::String
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    thek::mem::init::setup_mem();
    main();
    loop {}
}

fn main() {
    let v = vec!(1,2,3,4,5,6,7,8,9);
    for (i, x) in v.iter().enumerate() {
        print!("[{}] = {}, ", i, x);
    }
    println!();

    let s = String::from("Aix`o mola molt! e's guay!");
    println!("{}", s);

    let v2 = vec!(1,2,3,4,5,6,7,8,9);
    for (i, x) in v2.iter().enumerate() {
        print!("[{}] = {}, ", i, x);
    }
    println!();

    print!("\nHola");
    print_one();
    print_two();
    print_count(1);
    print!("Adeu!");
    println!();

    {
        let mut con = ScreenConsoleController::new(AnsiColor::BrightWhite, AnsiColor::BrightBlue);
        con.set_xy(34, 12).unwrap_or_default();
        w_print!(con, "<<< The K >>>");
    }

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