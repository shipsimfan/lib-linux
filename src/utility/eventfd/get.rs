use crate::EventFd;
use std::ffi::c_int;

impl EventFd {
    /// Get the underlying event handle
    pub const fn handle(&self) -> c_int {
        self.handle
    }
}
