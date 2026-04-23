use crate::{EPoll, unistd::close};

impl Drop for EPoll {
    fn drop(&mut self) {
        unsafe { close(self.handle) };
    }
}
