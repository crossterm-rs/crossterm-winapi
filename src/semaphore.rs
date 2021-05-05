use crate::Handle;
use std::{io, ptr};
use winapi::um::synchapi::{CreateSemaphoreW, ReleaseSemaphore};

/// A [Windows semaphore](https://docs.microsoft.com/en-us/windows/win32/sync/semaphore-objects).
#[derive(Clone, Debug)]
pub struct Semaphore(Handle);

impl Semaphore {
    /// Construct a new semaphore.
    ///
    /// This wraps
    /// [`CreateSemaphoreW`](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-createsemaphorew).
    pub fn new() -> io::Result<Self> {
        let handle = unsafe { CreateSemaphoreW(ptr::null_mut(), 0, 1, ptr::null_mut()) };

        if handle == ptr::null_mut() {
            return Err(io::Error::last_os_error());
        }

        let handle = unsafe { Handle::from_raw(handle) };
        Ok(Self(handle))
    }

    /// Release a permit on the semaphore.
    ///
    /// This wraps
    /// [`ReleaseSemaphore`](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-releasesemaphore).
    pub fn release(&self) -> io::Result<()> {
        let result = unsafe { ReleaseSemaphore(*self.0, 1, ptr::null_mut()) };

        if result == 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    /// Access the underlying handle to the semaphore.
    pub fn handle(&self) -> &Handle {
        &self.0
    }
}

unsafe impl Send for Semaphore {}

unsafe impl Sync for Semaphore {}
