use crate::EventFd;
use std::ffi::c_int;

impl EventFd {
    /// Get the underlying epoll handle
    pub fn handle(&self) -> c_int {
        self.handle
    }
}
