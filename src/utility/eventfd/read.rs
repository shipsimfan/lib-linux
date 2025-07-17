use crate::{try_linux, unistd::read, EventFd, Result};

impl EventFd {
    /// Read the value of the counter
    ///
    /// If blocking, this call will block until the counter contains a non-zero value.
    pub fn read(&self) -> Result<u64> {
        let mut count = 0;
        try_linux!(read(
            self.handle,
            &mut count as *mut _ as _,
            std::mem::size_of::<u64>() as _
        ))
        .map(|_| count)
    }

    /// Wait until the counter becomes non-zero
    ///
    /// If the [`EventFd`] is non-blocking, this function will return immediately.
    pub fn wait(&self) -> Result<()> {
        self.read().map(|_| ())
    }
}
