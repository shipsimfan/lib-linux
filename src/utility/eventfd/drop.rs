use crate::{EventFd, unistd::close};

impl Drop for EventFd {
    fn drop(&mut self) {
        unsafe { close(self.handle) };
    }
}
