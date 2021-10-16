#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::convert::From;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_title(b"-- Rust Kernel Test --");
    loop {}
}

fn print_title(msg: &[u8]) {
    let center_offs = 40 - msg.len() / 2;
    CONSOLE.print(msg, center_offs);
}

#[derive(Copy, Clone)]
enum ConsoleColor {
    Black = 0,
    Blue,
    Green,
    Cyan,
    Red,
    Purple,
    Brown,
    Gray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    LightPurple,
    Yellow,
    White
}

impl From<u8> for ConsoleColor {
    fn from(value: u8) -> Self {
        match value & 0xF {
            0 => ConsoleColor::Black,
            1 => ConsoleColor::Blue,
            2 => ConsoleColor::Green,
            3 => ConsoleColor::Cyan,
            4 => ConsoleColor::Red,
            5 => ConsoleColor::Purple,
            6 => ConsoleColor::Brown,
            7 => ConsoleColor::Gray,
            8 => ConsoleColor::DarkGray,
            9 => ConsoleColor::LightBlue,
            10 => ConsoleColor::LightGreen,
            11 => ConsoleColor::LightCyan,
            12 => ConsoleColor::LightRed,
            13 => ConsoleColor::LightPurple,
            14 => ConsoleColor::Yellow,
            15 => ConsoleColor::White,
            _ => ConsoleColor::Black
        }
    }
}

struct ConsoleChar {
    character: u8,
    color: ConsoleColor,
    bg_color: ConsoleColor
}

impl ConsoleChar {
    pub fn new(character: u8, color: ConsoleColor, bg_color: ConsoleColor) -> Self {
        Self {
            character,
            color,
            bg_color
        }
    }

    pub fn get_char(&self) -> u8 {
        self.character
    }

    pub fn get_color(&self) -> u8 {
        ((self.bg_color as u8) << 4) | (self.color as u8)
    }
}

/// Console struct with default colors for foreground and background
struct Console(ConsoleColor, ConsoleColor);

impl Console {
    pub fn set_char(&self, pos: usize, character: u8) -> Result<(), &'static str> {
        self.set_console_char(pos, ConsoleChar::new(character, self.0, self.1))
    }

    pub fn set_console_char(&self, pos: usize, character: ConsoleChar) -> Result<(), &'static str> {
        if pos < 2000 {
            unsafe {
                *((0xB8000 + pos * 2) as *mut u8) = character.get_char();
                *((0xB8000 + pos * 2 + 1) as *mut u8) = character.get_color();
            }
            Ok(())
        }
        else {
            Err("Position out of bounds")
        }
    }

    pub fn print(&self, msg: &[u8], pos: usize) {
        for (i, ch) in msg.iter().enumerate() {
            self.set_char(i + pos, *ch).unwrap_or(());
        }
    }
}

static CONSOLE : Console = Console(ConsoleColor::White, ConsoleColor::Red);
