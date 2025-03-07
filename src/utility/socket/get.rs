use crate::Socket;
use std::ffi::c_int;

impl Socket {
    /// Get the underlying socket handle
    pub fn handle(&self) -> c_int {
        self.handle
    }
}
