use crate::{
    Result, Socket,
    sys::socket::{SOMAXCONN, listen},
    try_linux,
};

impl Socket {
    /// Set the socket into the "LISTEN" state to begin accepting clients
    pub fn listen(&mut self, backlog: Option<usize>) -> Result<()> {
        try_linux!(listen(self.handle, backlog.unwrap_or(SOMAXCONN as _) as _)).map(|_| ())
    }
}
