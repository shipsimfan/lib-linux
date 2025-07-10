use crate::{AsHandle, Socket};
use std::ffi::c_int;

impl AsHandle for Socket {
    fn as_handle(&self) -> c_int {
        self.handle
    }
}
