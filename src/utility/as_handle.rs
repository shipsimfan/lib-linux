use std::ffi::c_int;

/// An item which wraps a linux handle
pub trait AsHandle {
    /// Get the underlying linux handle
    fn as_handle(&self) -> c_int;
}

impl AsHandle for c_int {
    fn as_handle(&self) -> c_int {
        *self
    }
}
