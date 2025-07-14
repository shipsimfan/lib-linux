use crate::{sys::socket::send, try_linux, Result, Socket};
use std::ffi::c_int;

impl Socket {
    /// Write data to a socket
    ///
    /// This function should be used on stream sockets
    pub fn write(&mut self, buf: &[u8], flags: c_int) -> Result<usize> {
        self.send(buf, flags)
    }

    /// Send a packet on a socket
    ///
    /// This function should be used on datagram and sequential packet sockets
    pub fn send(&self, buf: &[u8], flags: c_int) -> Result<usize> {
        try_linux!(send(self.handle, buf.as_ptr().cast(), buf.len(), flags)).map(|bytes| bytes as _)
    }
}
