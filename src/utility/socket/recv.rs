use crate::{sys::socket::recv, try_linux, Result, Socket};
use std::ffi::c_int;

impl Socket {
    /// Read data from a socket
    ///
    /// This function should be used on stream sockets
    pub fn read(&mut self, buf: &mut [u8], flags: c_int) -> Result<usize> {
        self.recv(buf, flags)
    }

    /// Read a packet from a socket
    ///
    /// This function should be used on datagram and sequential packet sockets
    pub fn recv(&self, buf: &mut [u8], flags: c_int) -> Result<usize> {
        try_linux!(recv(self.handle, buf.as_mut_ptr().cast(), buf.len(), flags))
            .map(|bytes| bytes as _)
    }
}
