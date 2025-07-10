use crate::{sys::socket::recv, try_linux, Result, Socket};
use std::ffi::c_int;

impl Socket {
    /// Receive a message from a socket
    pub fn recv(&mut self, buf: &mut [u8], flags: c_int) -> Result<usize> {
        try_linux!(recv(self.handle, buf.as_mut_ptr().cast(), buf.len(), flags))
            .map(|bytes| bytes as _)
    }
}
