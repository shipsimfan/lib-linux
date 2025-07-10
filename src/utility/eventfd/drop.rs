use crate::{unistd::close, EventFd};

impl Drop for EventFd {
    fn drop(&mut self) {
        unsafe { close(self.handle) };
    }
}
