use crate::{Result, Socket, SocketAddress, sys::socket::connect, try_linux};

impl Socket {
    /// Initiate a connection on a socket
    pub fn connect(&mut self, address: &SocketAddress) -> Result<()> {
        try_linux!(connect(self.handle, address.as_ptr(), address.len() as _)).map(|_| ())
    }
}
