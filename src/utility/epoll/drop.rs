use crate::{unistd::close, EPoll};

impl Drop for EPoll {
    fn drop(&mut self) {
        unsafe { close(self.handle) };
    }
}
