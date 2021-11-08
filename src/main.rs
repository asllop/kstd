#![no_main]
#![no_std]

use thek::{
    DefaultConsoleController,
    controllers::text::ansi::AnsiColor,
    mem::{
        arch::raw_mem,
        layout::{
            MemBlockSet,
        }
    },
    sys::{
        KMutex
    },
    devices::{
        Device,
        com::port::uart::{
            UartDevice, UartParity, UartSpeed
        }
    }
};

use core::default::Default;
use std::{
    prelude::v1::*,
    fmt::{
        Write
    },
    mem::size_of
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    _small_allocs_mem();
    main();
    loop {}
}

/// Setup memory schema optimized for small allocations
fn _small_allocs_mem() {
    thek::mem::init::setup_mem(&[
        (256, 90),              // 90% of mem in segments of 256Bytes
        (usize::MAX, 10)        // Remaining 10% in one segment
    ]);
}

/// Setup memory schema optimized for big allocations
fn _big_allocs_mem() {
    thek::mem::init::setup_mem(&[
        (4*1024, 10),       // 10% of mem in segments of 4KB
        (usize::MAX, 90)    // Remaining 90% in one single segment
    ]);
}

/*
TODO: create tests for mem
- Check used mem after every alloc/dealloc
- Check if segments are correctly selected depending on requiered size.
- Check that blocks are not overlaping.
*/

struct TestStruct {
    pub num: i32
}

static TEST : KMutex<TestStruct> = KMutex::new(TestStruct {
    num: 10
});

fn main() {

    let mut tst = TEST.acquire();
    print!("{} ", tst.num);
    tst.num = 50;
    print!("{}", tst.num);

    let mut con = DefaultConsoleController::new(AnsiColor::BrightWhite, AnsiColor::BrightBlue);
    con.clear();
    con.set_xy(33, 0).unwrap_or_default();
    write!(&mut con, " <<< TheK >>>").unwrap_or_default();
    core::mem::drop(con);

    print!("\n\n");

    let block_set = unsafe {
        let (ptr, _) = raw_mem();
        &mut*(ptr as *mut MemBlockSet)
    };

    for i in 0..block_set.len() {
        let blay = block_set.block_at(i).unwrap();
        println!("({})\t{:#x}\tseg={}\tblck={}\tstck={}\tpyld={}",
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

    print_count(3);

    /*
    let mut v = Vec::new();
    for _ in 0..1000 {
        v.push(String::from("Nova cadena"));
    }
    */

    // Print a backspace to remove the 'A'
    print!("\nHOLA\x08");

    let mut port = UartDevice::mutex().acquire();
    port.config(0, UartParity::None, 8, 1, UartSpeed::Baud9600);
    for i in 0..10 {
        write!(&mut port, "Loop {}\n", i).unwrap_or_default();
    }
    write!(&mut port, "\n\x1b[10CHOLA\n").unwrap_or_default();

    //_fail_unwrap();
    //_fail_oom_big_allocs();
    //_fail_oom_small_allocs();
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

struct Test {
    _num: usize,
    _arr: [u8; 10]
}

impl Default for Test {
    fn default() -> Self {
        Self {
            _num: 0,
            _arr: [0; 10]
        }
    }
}

fn _fail_oom_big_allocs() {
    // Crash, out of memory!
    let mut v = Vec::new();
    loop {
        v.push(Test::default());
    }
}


fn _fail_oom_small_allocs() {
    // Crash, out of memory!
    let mut v = Vec::new();
    loop {
        v.push(String::from("This is a string with many chars ...... .... .... ....... ....... ...... ........ ..... .... .. finish!"));
    }
}
