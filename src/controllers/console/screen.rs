use core::{
    fmt::{
        Write,
        Error
    },
    default::Default
};

use crate::{
    controllers::{
        console::{
            ansi::{
                AnsiColor
            }
        }
    },
    devices::{
        plot::{
            text::{
                ScreenTextDevice, STDOUT_DEVICE, PlotTextDeviceApi
            }
        }
    }
};

use crate::sys::{
    KLock, KError
};

use super::ConsoleController;

/// Screen console controller
pub struct ScreenConsoleController<'a> {
    cols: usize,
    rows: usize,
    x: usize,
    y: usize,
    console_lock: KLock<'a, ScreenTextDevice>,
    text_color: AnsiColor,
    bg_color: AnsiColor
}

impl ScreenConsoleController<'_> {
    pub fn new(text_color: AnsiColor, bg_color: AnsiColor) -> Self {
        let console_lock = STDOUT_DEVICE.lock();
        console_lock.enable_cursor().unwrap_or(());
        let (cols, rows) = console_lock.get_size().unwrap_or((0,0));
        let (x, y) = console_lock.get_cursor().unwrap_or((0,0));
        Self {
            cols, rows,
            x, y,
            console_lock,
            text_color, bg_color
        }
    }

    fn pos(&self) -> usize {
        self.cols * self.y + self.x
    }

    fn inc_pos(&mut self) {
        self.x += 1;
        if self.x >= self.cols {
            self.x = 0;
            self.y += 1;
        }
        if self.y >= self.rows {
            self.y = 0;
        }
    }

    fn line_break(&mut self) {
        self.y += 1;
        self.x = 0;
        if self.pos() >= self.cols * self.rows {
            self.x = 0;
            self.y = 0;
        }
    }
}

impl ConsoleController for ScreenConsoleController<'_> {

    fn get_xy(&self) -> (usize, usize) { (self.x, self.y) }

    fn set_xy(&mut self, x: usize, y: usize) -> Result<(), KError> {
        self.x = x;
        self.y = y;
        let (_, text_color, bg_color) = self.console_lock.read(x, y)?;
        if let (AnsiColor::Black, AnsiColor::Black) = (text_color, bg_color) {
            self.console_lock.set_color(x, y, self.text_color, self.bg_color)?;
        }
        self.console_lock.set_cursor(x, y)?;
        Ok(())
    }

    fn get_size(&self) -> (usize, usize) { (self.cols, self.rows) } 
}

impl Default for ScreenConsoleController<'_> {
    fn default() -> Self {
        Self::new(AnsiColor::White, AnsiColor::Black)
    }
}

//TODO: create a buffer and scroll all lines up when a new line happens
//TODO: parse ANSI commands in the string to set colors, etc

impl Write for ScreenConsoleController<'_> {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        for ch in s.as_bytes() {
            if *ch == 0x0Au8 {
                self.line_break();
            }
            else {
                if let Err(_) = self.console_lock.print(self.x, self.y, self.text_color, self.bg_color, *ch) {
                    return Err(Error);
                }
                self.inc_pos();
            }
        }
        if let Err(_) = self.set_xy(self.x, self.y) {
            return Err(Error);
        }
        Ok(())
    }
}
