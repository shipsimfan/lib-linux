use crate::{sys::socket::bind, try_linux, Result, Socket, SocketAddress};

impl Socket {
    /// Bind a socket to a name
    pub fn bind(&mut self, address: &SocketAddress) -> Result<()> {
        try_linux!(bind(self.handle, address.as_ptr(), address.len() as _)).map(|_| ())
    }
}
