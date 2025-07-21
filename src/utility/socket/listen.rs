use crate::{
    sys::socket::{listen, SOMAXCONN},
    try_linux, Result, Socket,
};

impl Socket {
    /// Set the socket into the "LISTEN" state to begin accepting clients
    pub fn listen(&mut self, backlog: Option<usize>) -> Result<()> {
        try_linux!(listen(self.handle, backlog.unwrap_or(SOMAXCONN as _) as _)).map(|_| ())
    }
}
