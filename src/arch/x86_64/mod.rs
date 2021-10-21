#[cfg(any(target_arch = "x86_64"))]
pub fn inb() {
    //TODO
}

#[cfg(any(target_arch = "x86_64"))]
pub fn outb(port: u16, data: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") data);
    }
}