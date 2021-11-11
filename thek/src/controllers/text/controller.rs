use core::{
    fmt::{
        Write,
        Error
    },
    default::Default
};

use crate::devices::{
    self, Device,
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
    fn get_device(id: &str) -> Result<Device, KError> {
        if let Some(d) = devices::get_text_device(id) {
            Ok(d)
        }
        else {
            Err(KError::Other)
        }
    }

    pub fn new(text_color: AnsiColor, bg_color: AnsiColor, device_id: String) -> Result<Self, KError> {
        let device = Self::get_device(&device_id)?;
        let text_dev = device.unwrap_text();
        text_dev.config_cursor(true, CursorShape::Default, CursorBlink::Default)?;
        let (cols, rows) = text_dev.size()?;
        let (x, y) = text_dev.get_position()?;
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
        let device = Self::get_device(&self.device_id)?;
        let text_dev = device.unwrap_text();
        self.x = x;
        self.y = y;
        let (_, text_color, bg_color) = text_dev.read(x, y)?;
        if let (AnsiColor::Black, AnsiColor::Black) = (text_color, bg_color) {
            text_dev.put_color(x, y, self.text_color, self.bg_color)?;
        }
        text_dev.set_position(x, y)?;
        Ok(())
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.cols, self.rows)
    }

    pub fn clear(&mut self) -> Result<(), KError> {
        let device = Self::get_device(&self.device_id)?;
        let text_dev = device.unwrap_text();
        for y in 0..self.rows {
            for x in 0..self.cols {
                text_dev.write(x, y, self.text_color, self.bg_color, ' ')?;
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
        let device = Self::get_device(&self.device_id)?;
        let text_dev = device.unwrap_text();
        // Copy all lines one line up (from 1 to rows-1)
        for line_num in 1..self.rows {
            for char_num in 0..self.cols {
                if let Ok((ch, text_color, bg_color)) = text_dev.read(char_num, line_num) {
                    text_dev.write(char_num, line_num - 1, text_color, bg_color, ch)?;
                }
            }
        }
        // Set last line empty
        for char_num in 0..self.cols {
            text_dev.write(char_num, self.rows - 1, self.text_color, self.bg_color, 0u8.into_char()?)?;
        }

        Ok(())
    }

    // TODO: convert from unicode to any encoding we have configured in the kernel
    // Default for VGA console is ASCII extended:
    // https://upload.wikimedia.org/wikipedia/commons/2/26/Ascii-codes-table.png
    fn internal_print(&mut self, ch: u8) -> Result<(), KError> {
        let device = Self::get_device(&self.device_id)?;
        let text_dev = device.unwrap_text();
        if let Err(_) = text_dev.write(self.x, self.y, self.text_color, self.bg_color, ch.into_char()?) {
            return Err(KError::Other);
        }
        Ok(())
    }
}

impl Default for TextController {
    fn default() -> Self {
        Self::new(AnsiColor::White, AnsiColor::Black, "CON1".to_owned())
            .expect("Device CON1 could not be acquired")
    }
}

impl Write for TextController {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        for ch in s.chars() {
            match ch as u32 {
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
                    let tab_num = self.x / 8;
                    self.x = (tab_num  + 1) * 8;
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
                    // ANSI Escape Sequence
                    // TODO: parse ANSI commands and perform action
                },
                _ => {
                    // Everything else is considered a printable char (even if it's not)
                    if self.internal_print(ch.into_ascii().unwrap_or_default()).is_err() {
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
