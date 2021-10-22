#![no_main]
#![no_std]

use blog_os::{
    print, println,
    controllers::{
        console:: {
            ScreenConsole
        }
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
    let i = unsafe {
        *(0xB8000 as *mut u8)
    };

    let a = [1,2,3];
    let _x = a[i as usize];
}