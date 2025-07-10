use crate::{AsHandle, EPoll};
use std::ffi::c_int;

impl AsHandle for EPoll {
    fn as_handle(&self) -> c_int {
        self.handle
    }
}
