use core::{
    fmt::{
        Write,
        Error
    },
    default::Default
};

use crate::devices::{
    InputFlow, OutputFlow,
    console::{
        ConCmd, ConCmdResult, ConsoleDevice, CON_DEVICE, AnsiColor
    }
};

use crate::sys::{
    KLock, KError
};

use super::ConsoleController;

/// Screen console controller
pub struct ScreenConsole<'a> {
    cols: usize,
    rows: usize,
    x: usize,
    y: usize,
    console_lock: KLock<'a, ConsoleDevice>,
    text_color: AnsiColor,
    bg_color: AnsiColor
}

impl ScreenConsole<'_> {
    pub fn new(text_color: AnsiColor, bg_color: AnsiColor) -> Self {
        let console_lock = CON_DEVICE.lock();
        let size = console_lock.read_cmd(
            ConCmd::GetSize
        ).unwrap_or(ConCmdResult::Size(0,0));
        //TODO: get current cursor position to init x,y
        if let ConCmdResult::Size(cols, rows) = size {
            Self {
                cols, rows,
                x: 0, y: 0,
                console_lock,
                text_color,
                bg_color
            }
        }
        else {
            panic!("Unexpected result of console command");
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

impl ConsoleController for ScreenConsole<'_> {

    fn get_xy(&self) -> (usize, usize) { (self.x, self.y) }

    //TODO: move cursor
    fn set_xy(&mut self, x: usize, y: usize) -> Result<(), KError> {
        self.x = x;
        self.y = y;
        Ok(())
    }

    fn get_size(&self) -> (usize, usize) { (self.cols, self.rows) } 
}

impl Default for ScreenConsole<'_> {
    fn default() -> Self {
        Self::new(AnsiColor::White, AnsiColor::Black)
    }
}

//TODO: create a buffer and scroll all lines up when a new line happens
//TODO: update cursor position
//TODO: parse ANSI commands in the string to set colors, etc

impl Write for ScreenConsole<'_> {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        for ch in s.as_bytes() {
            if *ch == 0x0Au8 {
                self.line_break();
            }
            else {
                self.console_lock.write_cmd(
                    //TODO: get color from ANSI commands
                    ConCmd::Print(self.x, self.y, self.text_color, self.bg_color),
                    *ch
                ).unwrap_or_default();
                self.inc_pos();
            }
        }
        Ok(())
    }
}
