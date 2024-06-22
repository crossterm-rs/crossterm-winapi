//! This contains the logic for working with the console buffer.

use std::{io::Result, mem::size_of};

use windows::Win32::{
    Foundation::{GENERIC_READ, GENERIC_WRITE},
    Security::SECURITY_ATTRIBUTES,
    Storage::FileSystem::{FILE_SHARE_READ, FILE_SHARE_WRITE},
    System::Console::{
        CreateConsoleScreenBuffer, GetConsoleScreenBufferInfo, GetCurrentConsoleFont,
        SetConsoleActiveScreenBuffer, SetConsoleScreenBufferSize, CONSOLE_TEXTMODE_BUFFER, COORD,
    },
};

use super::{FontInfo, Handle, HandleType, ScreenBufferInfo};

/// A wrapper around a screen buffer.
#[derive(Clone, Debug)]
pub struct ScreenBuffer {
    handle: Handle,
}

impl ScreenBuffer {
    /// Create a wrapper around a screen buffer from its handle.
    pub fn new(handle: Handle) -> Self {
        Self { handle }
    }

    /// Get the current console screen buffer
    pub fn current() -> Result<ScreenBuffer> {
        Ok(ScreenBuffer {
            handle: Handle::new(HandleType::CurrentOutputHandle)?,
        })
    }

    /// Create new console screen buffer.
    ///
    /// This wraps
    /// [`CreateConsoleScreenBuffer`](https://docs.microsoft.com/en-us/windows/console/createconsolescreenbuffer)
    pub fn create() -> Result<ScreenBuffer> {
        let security_attr: SECURITY_ATTRIBUTES = SECURITY_ATTRIBUTES {
            nLength: size_of::<SECURITY_ATTRIBUTES>() as u32,
            lpSecurityDescriptor: ::std::ptr::null_mut(),
            bInheritHandle: true.into(),
        };

        let new_screen_buffer = unsafe {
            CreateConsoleScreenBuffer(
                (GENERIC_READ | GENERIC_WRITE).0,       // read/write access
                (FILE_SHARE_READ | FILE_SHARE_WRITE).0, // shared
                Some(&security_attr),                   // security attributes
                CONSOLE_TEXTMODE_BUFFER,                // must be TEXTMODE
                None,                                   // no existing screen buffer to copy
            )
        }?;
        Ok(ScreenBuffer {
            handle: unsafe { Handle::from_raw(new_screen_buffer) },
        })
    }

    /// Set this screen buffer to the current one.
    ///
    /// This wraps
    /// [`SetConsoleActiveScreenBuffer`](https://docs.microsoft.com/en-us/windows/console/setconsoleactivescreenbuffer).
    pub fn show(&self) -> Result<()> {
        unsafe { SetConsoleActiveScreenBuffer(*self.handle) }?;
        Ok(())
    }

    /// Get the screen buffer information like terminal size, cursor position, buffer size.
    ///
    /// This wraps
    /// [`GetConsoleScreenBufferInfo`](https://docs.microsoft.com/en-us/windows/console/getconsolescreenbufferinfo).
    pub fn info(&self) -> Result<ScreenBufferInfo> {
        let mut csbi = ScreenBufferInfo::new();
        unsafe { GetConsoleScreenBufferInfo(*self.handle, &mut csbi.0) }?;
        Ok(csbi)
    }

    /// Get the current font information like size and font index.
    ///
    /// This wraps
    /// [`GetCurrentConsoleFont`](https://learn.microsoft.com/en-us/windows/console/getcurrentconsolefont).
    pub fn font_info(&self) -> Result<FontInfo> {
        let mut font_info = FontInfo::new();
        unsafe {
            GetCurrentConsoleFont(
                *self.handle,
                false, // get info for current window size not the maximum window size
                &mut font_info.0,
            )
        }?;
        Ok(font_info)
    }

    /// Set the console screen buffer size to the given size.
    ///
    /// This wraps
    /// [`SetConsoleScreenBufferSize`](https://docs.microsoft.com/en-us/windows/console/setconsolescreenbuffersize).
    pub fn set_size(&self, x: i16, y: i16) -> Result<()> {
        unsafe { SetConsoleScreenBufferSize(*self.handle, COORD { X: x, Y: y }) }?;
        Ok(())
    }

    /// Get the underlying raw `HANDLE` used by this type to execute with.
    pub fn handle(&self) -> &Handle {
        &self.handle
    }
}

impl From<Handle> for ScreenBuffer {
    fn from(handle: Handle) -> Self {
        ScreenBuffer { handle }
    }
}

#[cfg(test)]
mod tests {
    use super::ScreenBuffer;

    #[test]
    fn test_screen_buffer_info() {
        let buffer = ScreenBuffer::current().unwrap();
        let info = buffer.info().unwrap();
        info.terminal_size();
        info.terminal_window();
        info.attributes();
        info.cursor_pos();
    }
}
