use crate::{Result, Socket, SocketAddress, sys::socket::bind, try_linux};

impl Socket {
    /// Bind a socket to a name
    pub fn bind(&mut self, address: &SocketAddress) -> Result<()> {
        try_linux!(bind(self.handle, address.as_ptr(), address.len() as _)).map(|_| ())
    }
}
