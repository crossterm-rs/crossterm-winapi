use std::{io, ptr};
use winapi::um::{
    handleapi::CloseHandle,
    synchapi::{CreateSemaphoreW, ReleaseSemaphore},
    winnt::HANDLE,
};

pub struct Semaphore(HANDLE);

impl Semaphore {
    /// Construct a new semaphore.
    pub fn new() -> io::Result<Self> {
        let handle = unsafe { CreateSemaphoreW(ptr::null_mut(), 0, 1, ptr::null_mut()) };

        if handle == ptr::null_mut() {
            return Err(io::Error::last_os_error());
        }

        Ok(Self(handle))
    }

    /// Release a permit on the semaphore.
    pub fn release(&self) -> io::Result<()> {
        let result = unsafe { ReleaseSemaphore(self.0, 1, ptr::null_mut()) };

        if result == 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    /// Access the underlying handle to the semaphore.
    pub fn handle(&self) -> HANDLE {
        self.0
    }
}

impl Drop for Semaphore {
    fn drop(&mut self) {
        assert!(
            unsafe { CloseHandle(self.0) } != 0,
            "failed to close handle"
        );
    }
}

unsafe impl Send for Semaphore {}

unsafe impl Sync for Semaphore {}
