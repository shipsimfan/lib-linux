use crate::{
    sys::socket::{listen, SOMAXCONN},
    try_linux, Result, Socket,
};

impl Socket {
    /// Listen for connections on a socket
    pub fn listen(&mut self, backlog: Option<usize>) -> Result<()> {
        try_linux!(listen(self.handle, backlog.unwrap_or(SOMAXCONN as _) as _)).map(|_| ())
    }
}
