use core::{
    fmt::{
        Write,
        Error
    }
};

use crate::devices::{
    InputFlow, OutputFlow,
    console::{
        ConCmd, ConCmdResult, ConsoleDevice, CON_DEVICE, AnsiColor
    }
};

use crate::sys::{
    KMutex as Mutex, KLock as Lock
};

/*
//TODO: define proper socket types
type TcpSocket = ();
type UdpSocket = ();

/// Console type
pub enum ConsoleType {
    /// Screen, using the graphic card
    Screen,
    /// Serial port (port number)
    Serial(usize),
    /// UDP socket
    Udp(UdpSocket),
    /// TCP socket
    Tcp(TcpSocket)
}
*/

pub struct ConsoleController<'a> {
    cols: usize,
    rows: usize,
    x: usize,
    y: usize,
    console_lock: Lock<'a, ConsoleDevice>,
    text_color: AnsiColor,
    bg_color: AnsiColor
}

impl ConsoleController<'_> {
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

    pub fn x(&self) -> usize { self.x }

    pub fn y(&self) -> usize { self.y }

    pub fn rows(&self) -> usize { self.rows }

    pub fn cols(&self) -> usize { self.cols }

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


impl core::default::Default for ConsoleController<'_> {
    fn default() -> Self {
        Self::new(AnsiColor::White, AnsiColor::Black)
    }
}

//TODO: create a buffer and scroll all lines up when a new line happens
//TODO: update cursor position
//TODO: parse ANSI commands in the string to set colors, etc

impl Write for ConsoleController<'_> {
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
