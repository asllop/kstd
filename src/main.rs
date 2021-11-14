#![no_main]
#![no_std]

use thek::{
    mem::{
        arch::raw_mem,
        layout::{
            MemBlockSet,
        }
    },
    // devices::{
    //     self
    // },
    devices::text::ansi::AnsiColor,
    controllers::{
        stdout::StdoutController,
        port::PortController,
        text::TextController
    }
};

use core::default::Default;
use std::{
    prelude::v1::*,
    fmt::{
        Write
    },
    collections::HashMap,
    mem::size_of
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    thek::cpu::init();
    small_allocs_mem();
    thek::devices::init_devices();

    main();
    print!(".END.");
    loop {}
}

/// Setup memory schema optimized for small allocations
fn small_allocs_mem() {
    thek::mem::init::setup_mem(&[
        (256, 80),              // 80% of mem in segments of 256Bytes
        (1024, 10),             // 10% of mem in segments of 1K
        (usize::MAX - 1, 5),    // Remaining 10% in two segments
        (usize::MAX, 5)
    ]);
}

/*
TODO: create tests for mem
- Check used mem after every alloc/dealloc
- Check if segments are correctly selected depending on requiered size.
- Check that blocks are not overlaping.
*/

fn _main() {
    println!("Hola");
    println!();
    println!("Nom\tGen\tEdat");
    println!("---------------------");
    println!("Andreu\tM\t37");
    println!("Blanca\tF\t36");
    println!("Mar\tF\t2");
    println!();
}

fn main() {
    println!("Hola!");
    let mut con = TextController::new(
        AnsiColor::BrightWhite, AnsiColor::BrightBlue, "CON1".to_owned()
    ).unwrap();
    con.clear().unwrap_or_default();
    con.set_xy(33, 0).unwrap_or_default();
    write!(&mut con, " <<< TheK >>>").unwrap_or_default();

    print!("\n\n");

    let block_set = unsafe {
        let (ptr, _) = raw_mem();
        &mut*(ptr as *mut MemBlockSet)
    };

    for i in 0..block_set.len() {
        let blay = block_set.block_at(i).unwrap();
        println!("({}) ; {:#x} ; seg={} ; blck={} ; stck={} ; pyld={}",
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
    println!();

    /*
    // Using serial port device directly
    let device = devices::get_port_device("SER1").expect("Port SER1 not found");
    let port = device.unwrap_port();
    port.write('H' as u8).unwrap_or_default();
    port.write('o' as u8).unwrap_or_default();
    port.write('l' as u8).unwrap_or_default();
    port.write('a' as u8).unwrap_or_default();
    port.write('!' as u8).unwrap_or_default();
    port.write('\n' as u8).unwrap_or_default();

    print!("Enter string: ");
    let mut vec = Vec::<u8>::new();
    loop {
        let ch = port.read().unwrap_or_default();
        if ch == '\n' as u8 || ch == 0x0Du8 {
            break;
        }
        vec.push(ch);
    }
    let input = String::from_utf8(vec).unwrap();
    println!("Input = {}", input);
    // unlock port device
    core::mem::drop(port);
    */

    // Using serial port device through a controller

    StdoutController::set(Box::new(PortController::default()));

    println!("Hello");
    println!("this is using");
    println!("the port controller!");

    let mut map_1 = HashMap::new();
    map_1.insert("nom", "Andreu");
    map_1.insert("cognoms", "Santarén Llop");
    map_1.insert("edat", "37");

    let mut map_2 = HashMap::new();
    map_2.insert("nom", "Blanca");
    map_2.insert("cognoms", "Garcés Dueñas");
    map_2.insert("edat", "36");

    let mut map_3 = HashMap::new();
    map_3.insert("nom", "Mar");
    map_3.insert("cognoms", "Santarén Garcés");
    map_3.insert("edat", "2");

    let llista = vec!(map_1, map_2, map_3);

    println!("Llista =\n{:#?}", llista);

    //_fail_unwrap();
    //_fail_oom_big_allocs();
    //_fail_oom_small_allocs();
    //_force_fault();
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

fn _force_fault() {
    unsafe {
        *(0xffffffffffffffff as *mut u64) = 0;
    };
}