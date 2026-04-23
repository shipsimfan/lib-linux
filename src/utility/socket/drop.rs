use crate::{Socket, unistd::close};

impl Drop for Socket {
    fn drop(&mut self) {
        unsafe { close(self.handle) };
    }
}
