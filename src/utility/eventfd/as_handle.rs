use crate::{AsHandle, EventFd};
use std::ffi::c_int;

impl AsHandle for EventFd {
    fn as_handle(&self) -> c_int {
        self.handle
    }
}
