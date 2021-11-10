use core::{
    fmt::{
        Write,
        Error
    },
    default::Default
};

use crate::devices::{
    DeviceType, get_device_store,
    text::{
        CursorBlink,
        CursorShape,
        ansi::{
            AnsiColor, IntoAscii, IntoChar
        }
    }
};

use crate::sys::{
    KError
};

extern crate alloc;
use alloc::{
    borrow::ToOwned,
    string::String
};

/// Text controller.
pub struct TextController {
    cols: usize,
    rows: usize,
    x: usize,
    y: usize,
    device_id: String,
    text_color: AnsiColor,
    bg_color: AnsiColor
}

impl TextController {
    fn get_device(id: &str) -> Result<DeviceType, KError> {
        let device = get_device_store().get_text(id);
        let device = if device.is_none() {
            return Err(KError::Other);
        }
        else {
            device.unwrap()
        };
        Ok(device)
    }

    pub fn new(text_color: AnsiColor, bg_color: AnsiColor, device_id: String) -> Result<Self, KError> {
        let text_dev = Self::get_device(&device_id)?;
        text_dev.unwrap_text().config_cursor(true, CursorShape::Default, CursorBlink::Default)?;
        let (cols, rows) = text_dev.unwrap_text().size()?;
        let (x, y) = text_dev.unwrap_text().get_position()?;
        Ok(
            Self {
                cols, rows,
                x, y,
                device_id,
                text_color, bg_color
            }
        )
    }

    pub fn get_xy(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn set_xy(&mut self, x: usize, y: usize) -> Result<(), KError> {
        let text_dev = Self::get_device(&self.device_id)?;
        self.x = x;
        self.y = y;
        let (_, text_color, bg_color) = text_dev.unwrap_text().read(x, y)?;
        if let (AnsiColor::Black, AnsiColor::Black) = (text_color, bg_color) {
            text_dev.unwrap_text().put_color(x, y, self.text_color, self.bg_color)?;
        }
        text_dev.unwrap_text().set_position(x, y)?;
        Ok(())
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.cols, self.rows)
    }

    pub fn clear(&mut self) -> Result<(), KError> {
        let text_dev = Self::get_device(&self.device_id)?;
        for y in 0..self.rows {
            for x in 0..self.cols {
                text_dev.unwrap_text().write(x, y, self.text_color, self.bg_color, ' ')?;
            }
        }
        Ok(())
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

    fn scroll_up(&self) -> Result<(), KError> {
        let text_dev = Self::get_device(&self.device_id)?;
        // Copy all lines one line up (from 1 to rows-1)
        for line_num in 1..self.rows {
            for char_num in 0..self.cols {
                if let Ok((ch, text_color, bg_color)) = text_dev.unwrap_text().read(char_num, line_num) {
                    text_dev.unwrap_text().write(char_num, line_num - 1, text_color, bg_color, ch)?;
                }
            }
        }
        // Set last line empty
        for char_num in 0..self.cols {
            text_dev.unwrap_text().write(char_num, self.rows - 1, self.text_color, self.bg_color, 0u8.into_char()?)?;
        }

        Ok(())
    }

    fn internal_print(&mut self, ch: u8) -> Result<(), KError> {
        let text_dev = Self::get_device(&self.device_id)?;
        if let Err(_) = text_dev.unwrap_text().write(self.x, self.y, self.text_color, self.bg_color, ch.into_char()?) {
            return Err(KError::Other);
        }
        Ok(())
    }
}

impl Default for TextController {
    fn default() -> Self {
        Self::new(AnsiColor::White, AnsiColor::Black, "TXT1".to_owned())
            .expect("Device TXT1 not found")
    }
}

impl Write for TextController {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        for ch in s.as_bytes() {
            match *ch {
                0x0a => {
                    // Newline
                    if self.y + 1 >= self.rows {
                        if self.scroll_up().is_err() {
                            return Err(Error);
                        }
                        self.x = 0;
                    }
                    else {
                        self.y += 1;
                        self.x = 0;
                    }
                },
                0x09 => {
                    // Tab
                    let tab_num = self.x / 4;
                    self.x = (tab_num  + 1) * 4;
                    if self.x >= self.cols {
                        self.x = self.cols - 1;
                    }
                },
                0x08 => {
                    // Backspace
                    if self.x > 0 {
                        self.x -= 1;
                    }
                    else {
                        if self.y > 0 {
                            self.x = self.cols - 1;
                            self.y -= 1;
                        }
                        else {
                            // X and Y are 0, screen origin, do nothing
                        }
                    }
                    // Print space to actually remove char
                    if self.internal_print(' '.into_ascii().unwrap()).is_err() {
                        return Err(Error);
                    }
                },
                0x1b => {
                    /*
                    // ANSI Escape Sequence
                    if self.device_lock.is_ansi() {
                        // The device can handle ANSI commands
                        self.internal_print(*ch)?;
                        self.inc_pos();
                    }
                    else {
                        //TODO: parse ANSI commands
                    }
                    */
                },
                _ => {
                    // Everything else is considered a printable char (even if it's not)
                    if self.internal_print(*ch).is_err() {
                        return Err(Error);
                    }
                    self.inc_pos();
                }
            }
        }
        if let Err(_) = self.set_xy(self.x, self.y) {
            return Err(Error);
        }
        Ok(())
    }
}
