#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_message(b"-- Bon dia! --");
    loop {}
}

unsafe fn set_video_data(index: i32, data: u8) {
    *((0xB8000 + index) as *mut u8) = data;
}

fn print_message(msg: &[u8]) {
    unsafe {
        let mut index = 0;
        for ch in msg {
            set_video_data(index, *ch);
            index += 1;
            set_video_data(index, 12); // light red
            index += 1;
        }
    }
}
