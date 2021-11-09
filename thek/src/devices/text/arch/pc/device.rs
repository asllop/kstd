//! PC VGA text device.

use crate::devices::{
    text::{
        Text, CursorShape, CursorBlink,
        ansi::{
            AnsiColor, IntoAscii, IntoChar
        }
    },
    Id, Interrupt, DeviceType, DeviceStore
};

use crate::arch::{
    inb, outb
};

use crate::sys::{
    KMutex, KError
};

//TODO: add macro mark
pub fn register_devices(device_store: &KMutex<DeviceStore>) {
    device_store.acquire().register_device(DeviceType::Text(&VGA_TEXT_DEVICE_1_MUTEX));
}

static VGA_TEXT_DEVICE_1 : VgaTextDevice = VgaTextDevice::new();
static VGA_TEXT_DEVICE_1_MUTEX : KMutex<&'static dyn Text> = KMutex::new(&VGA_TEXT_DEVICE_1);

/// Vga console colors
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum VgaConsoleColor {
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

/// Convert an ANSI color to an VGA console color code
impl From<AnsiColor> for VgaConsoleColor {
    fn from(value: AnsiColor) -> Self {
        match value {
            AnsiColor::Black => VgaConsoleColor::Black,
            AnsiColor::Blue => VgaConsoleColor::Blue,
            AnsiColor::Green => VgaConsoleColor::Green,
            AnsiColor::Cyan => VgaConsoleColor::Cyan,
            AnsiColor::Red => VgaConsoleColor::Red,
            AnsiColor::Magenta => VgaConsoleColor::Purple,
            AnsiColor::Yellow => VgaConsoleColor::Brown,
            AnsiColor::BrightWhite => VgaConsoleColor::White,
            AnsiColor::BrightBlack => VgaConsoleColor::DarkGray,
            AnsiColor::BrightBlue => VgaConsoleColor::LightBlue,
            AnsiColor::BrightGreen => VgaConsoleColor::LightGreen,
            AnsiColor::BrightCyan => VgaConsoleColor::LightCyan,
            AnsiColor::BrightRed => VgaConsoleColor::LightRed,
            AnsiColor::BrightMagenta => VgaConsoleColor::LightPurple,
            AnsiColor::BrightYellow => VgaConsoleColor::Yellow,
            AnsiColor::White => VgaConsoleColor::Gray,
            // First 16 colors are the ones supported by VgaConsoleColor. For the rest we map in blocks of 16.
            AnsiColor::Color256(c) => {
                if c < 16 {
                    VgaConsoleColor::from(c)
                }
                else {
                    VgaConsoleColor::from(c / 16u8)
                }
            }
        }
    }
}

impl From<VgaConsoleColor> for AnsiColor {
    fn from(value: VgaConsoleColor) -> Self {
        match value {
            VgaConsoleColor::Black => AnsiColor::Black,
            VgaConsoleColor::Blue => AnsiColor::Blue,
            VgaConsoleColor::Green => AnsiColor::Green,
            VgaConsoleColor::Cyan => AnsiColor::Cyan,
            VgaConsoleColor::Red => AnsiColor::Red,
            VgaConsoleColor::Purple => AnsiColor::Magenta,
            VgaConsoleColor::Brown => AnsiColor::Yellow,
            VgaConsoleColor::White => AnsiColor::BrightWhite,
            VgaConsoleColor::DarkGray => AnsiColor::BrightBlack,
            VgaConsoleColor::LightBlue => AnsiColor::BrightBlue,
            VgaConsoleColor::LightGreen => AnsiColor::BrightGreen,
            VgaConsoleColor::LightCyan => AnsiColor::BrightCyan,
            VgaConsoleColor::LightRed => AnsiColor::BrightRed,
            VgaConsoleColor::LightPurple => AnsiColor::BrightMagenta ,
            VgaConsoleColor::Yellow => AnsiColor::BrightYellow,
            VgaConsoleColor::Gray => AnsiColor::White
        }
    }
}

impl From<u8> for VgaConsoleColor {
    fn from(value: u8) -> Self {
        match value & 0xF {
            0 => VgaConsoleColor::Black,
            1 => VgaConsoleColor::Blue,
            2 => VgaConsoleColor::Green,
            3 => VgaConsoleColor::Cyan,
            4 => VgaConsoleColor::Red,
            5 => VgaConsoleColor::Purple,
            6 => VgaConsoleColor::Brown,
            7 => VgaConsoleColor::Gray,
            8 => VgaConsoleColor::DarkGray,
            9 => VgaConsoleColor::LightBlue,
            10 => VgaConsoleColor::LightGreen,
            11 => VgaConsoleColor::LightCyan,
            12 => VgaConsoleColor::LightRed,
            13 => VgaConsoleColor::LightPurple,
            14 => VgaConsoleColor::Yellow,
            15 => VgaConsoleColor::White,
            _ => VgaConsoleColor::Black
        }
    }
}

const CONSOLE_COLS : usize = 80;
const CONSOLE_ROWS : usize = 25;

/// VGA text mode device.
pub struct VgaTextDevice {
}

impl VgaTextDevice {
    const fn new() -> Self {
        Self {}
    }
}

impl Text for VgaTextDevice {
    fn write(&self, x: usize, y: usize, text_color: AnsiColor, bg_color: AnsiColor, ch: char) -> Result<(), KError> {
        if x < CONSOLE_COLS && y < CONSOLE_ROWS {
            let pos = CONSOLE_COLS * y + x;
            unsafe {
                *((0xB8000 + pos * 2) as *mut u8) = ch.into_ascii()?;
                let color = ((VgaConsoleColor::from(bg_color) as u8) << 4) | (VgaConsoleColor::from(text_color) as u8);
                *((0xB8000 + pos * 2 + 1) as *mut u8) = color;
            }
            Ok(())
        }
        else {
            Err(KError::OutBounds)
        }
    }

    fn put_char(&self, x: usize, y: usize, ch: char) -> Result<(), KError> {
        if x < CONSOLE_COLS && y < CONSOLE_ROWS {
            let pos = CONSOLE_COLS * y + x;
            unsafe {
                *((0xB8000 + pos * 2) as *mut u8) = ch.into_ascii()?;
            }
            Ok(())
        }
        else {
            Err(KError::OutBounds)
        }
    }

    fn put_color(&self, x: usize, y: usize, text_color: AnsiColor, bg_color: AnsiColor) -> Result<(), KError> {
        if x < CONSOLE_COLS && y < CONSOLE_ROWS {
            let pos = CONSOLE_COLS * y + x;
            unsafe {
                let color = ((VgaConsoleColor::from(bg_color) as u8) << 4) | (VgaConsoleColor::from(text_color) as u8);
                *((0xB8000 + pos * 2 + 1) as *mut u8) = color;
            }
            Ok(())
        }
        else {
            Err(KError::OutBounds)
        }
    }

    fn read(&self, x: usize, y: usize) -> Result<(char, AnsiColor, AnsiColor), KError> {
        if x < CONSOLE_COLS && y < CONSOLE_ROWS {
            let pos = CONSOLE_COLS * y + x;
            let ch = unsafe {
                *((0xB8000 + pos * 2) as *mut u8)
            };
            let raw_color = unsafe {
                *((0xB8000 + pos * 2 + 1) as *mut u8)
            };

            let text_color = VgaConsoleColor::from(raw_color & 0xF);
            let bg_color = VgaConsoleColor::from((raw_color >> 4) & 0xF);
            
            Ok(
                (
                    ch.into_char()?,
                    AnsiColor::from(text_color),
                    AnsiColor::from(bg_color)
                )
            )
        }
        else {
            Err(KError::OutBounds)
        }
    }

    fn set_position(&self, x: usize, y: usize) -> Result<(), KError> {
        if x < CONSOLE_COLS && y < CONSOLE_ROWS {
            let pos: u16 = (80 * y + x) as u16;
            outb(0x3D4, 0x0F);
            outb(0x3D5, (pos & 0xFF) as u8);
            outb(0x3D4, 0x0E);
            outb(0x3D5, ((pos >> 8) & 0xFF) as u8);
            Ok(())
        }
        else {
            Err(KError::OutBounds)
        }
    }

    fn get_position(&self) -> Result<(usize, usize), KError> {
        let mut pos : u16 = 0;
        outb(0x3D4, 0x0F);
        pos |= inb(0x3D5) as u16;
        outb(0x3D4, 0x0E);
        pos |= (inb(0x3D5) as u16) << 8;
        let y = (pos / 80) as usize;
        let x = (pos % 80) as usize;
        Ok((x, y))
    }

    fn config_cursor(&self, enabled: bool, shape: CursorShape, _blink: CursorBlink) -> Result<(), KError> {
        if enabled {
            let (cursor_start, cursor_end) = match shape {
                CursorShape::Default => {
                    (14, 15)
                },
                CursorShape::FullBlock => {
                    (0, 15)
                },
                CursorShape::HalfBlock => {
                    (7, 15)
                },
                CursorShape::UnderLine => {
                    (14, 15)
                }
            };
            outb(0x3D4, 0x0A);
            outb(0x3D5, (inb(0x3D5) & 0xC0) | cursor_start);
            outb(0x3D4, 0x0B);
            outb(0x3D5, (inb(0x3D5) & 0xE0) | cursor_end);
        }
        else {
            outb(0x3D4, 0x0A);
            outb(0x3D5, 0x20);
        }

        Ok(())
    }

    fn size(&self) -> Result<(usize, usize), KError> {
        Ok((CONSOLE_COLS, CONSOLE_ROWS))
    }
}

impl Id for VgaTextDevice {
    fn id(&self) -> &str {
        "TXT0"
    }
}

impl Interrupt for VgaTextDevice {
    fn handler(&self, _: fn(DeviceType)) -> bool { false }
}