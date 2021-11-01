//! Functions for x86_64 architecture.

/// Input byte from port
#[cfg(any(target_arch = "x86_64"))]
pub fn inb(port: u16) -> u8 {
    let r: u8;
    unsafe {
        asm!("in al, dx", out("al") r, in("dx") port);
    }
    r
}

/// Output byte to port
#[cfg(any(target_arch = "x86_64"))]
pub fn outb(port: u16, data: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") data);
    }
}

/// Halt
#[cfg(any(target_arch = "x86_64"))]
pub fn halt() {
    unsafe {
        asm!("cli");
        asm!("hlt");
    }
}