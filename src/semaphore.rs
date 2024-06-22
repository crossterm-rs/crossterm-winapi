use std::io;

use windows::Win32::System::Threading::{CreateSemaphoreW, ReleaseSemaphore};

use crate::Handle;

/// A [Windows semaphore](https://docs.microsoft.com/en-us/windows/win32/sync/semaphore-objects).
#[derive(Clone, Debug)]
pub struct Semaphore(Handle);

impl Semaphore {
    /// Construct a new semaphore.
    ///
    /// This wraps
    /// [`CreateSemaphoreW`](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-createsemaphorew).
    pub fn new() -> io::Result<Self> {
        let handle = unsafe {
            CreateSemaphoreW(
                None,                  // no security attributes
                0,                     // initial count
                1,                     // maximum count
                windows::core::w!(""), // unnamed semaphore
            )
        }?;
        let handle = unsafe { Handle::from_raw(handle) };
        Ok(Self(handle))
    }

    /// Release a permit on the semaphore.
    ///
    /// This wraps
    /// [`ReleaseSemaphore`](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-releasesemaphore).
    pub fn release(&self) -> io::Result<()> {
        let previous_count = None;
        unsafe { ReleaseSemaphore(*self.0, 1, previous_count) }?;
        Ok(())
    }

    /// Access the underlying handle to the semaphore.
    pub fn handle(&self) -> &Handle {
        &self.0
    }
}

unsafe impl Send for Semaphore {}

unsafe impl Sync for Semaphore {}
