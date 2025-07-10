use crate::{sys::socket::send, try_linux, Result, Socket};
use std::ffi::c_int;

impl Socket {
    /// Send a message on a socket
    pub fn send(&mut self, buf: &[u8], flags: c_int) -> Result<usize> {
        try_linux!(send(self.handle, buf.as_ptr().cast(), buf.len(), flags)).map(|bytes| bytes as _)
    }
}
