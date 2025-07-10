use crate::EPoll;
use std::ffi::c_int;

impl EPoll {
    /// Get the underlying epoll handle
    pub fn handle(&self) -> c_int {
        self.handle
    }
}
