use crate::{unistd::close, Socket};

impl Drop for Socket {
    fn drop(&mut self) {
        unsafe { close(self.handle) };
    }
}
