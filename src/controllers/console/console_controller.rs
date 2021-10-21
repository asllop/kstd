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
    console_lock: Lock<'a, ConsoleDevice>
}

impl ConsoleController<'_> {
    pub fn new() -> Self {
        let console_lock = CON_DEVICE.lock();
        let size = console_lock.read_cmd(
            ConCmd::GetSize
        ).unwrap_or(ConCmdResult::Size(0,0));
        //TODO: get current cursor position to init x,y
        if let ConCmdResult::Size(cols, rows) = size {
            Self {
                cols, rows,
                x: 0, y: 0,
                console_lock
            }
        }
        else {
            panic!("Unexpected result of console command");
        }
    }

    pub fn pos(&self) -> usize {
        self.cols * self.y + self.x
    }

    pub fn inc_pos(&mut self) {
        self.x += 1;
        if self.x >= self.cols {
            self.x = 0;
            self.y += 1;
        }
        if self.y >= self.rows {
            self.y = 0;
        }
    }

    pub fn line_break(&mut self) {
        self.y += 1;
        self.x = 0;
        if self.pos() >= self.cols * self.rows {
            self.x = 0;
            self.y = 0;
        }
    }
}

//TODO: create a buffer and scroll all lines up when a new line happens

impl Write for ConsoleController<'_> {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        for ch in s.as_bytes() {
            if *ch == 0x0Au8 {
                self.line_break();
            }
            else {
                self.console_lock.write_cmd(
                    ConCmd::Print(self.x, self.y, AnsiColor::BrightYellow, AnsiColor::BrightBlack),
                    *ch
                ).unwrap_or_default();
                self.inc_pos();
            }
        }
        Ok(())
    }
}
