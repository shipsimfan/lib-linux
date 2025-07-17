use crate::{try_linux, unistd::write, EventFd, Result};

impl EventFd {
    /// Write a value into the counter, waking threads if any are blocking
    pub fn write(&self, count: u64) -> Result<()> {
        try_linux!(write(
            self.handle,
            &count as *const _ as _,
            std::mem::size_of::<u64>() as _
        ))
        .map(|_| ())
    }

    /// Writes 1 into the counter, waking any threads waiting on this [`EventFd`]
    pub fn signal(&self) -> Result<()> {
        self.write(1)
    }
}
