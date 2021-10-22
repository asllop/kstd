#![no_main]
#![no_std]

use thek::{
    print, println, w_print,
    controllers::console::{
        ansi::AnsiColor,
        ScreenConsoleController, ConsoleController
    }
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    main();
    loop {}
}

fn main() {
    print!("Hola");
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