use std::ffi::c_int;

/// A linux descriptor of any type
pub trait Descriptor {
    /// Get the raw underlying descriptor
    unsafe fn as_raw_handle(&self) -> c_int;
}
