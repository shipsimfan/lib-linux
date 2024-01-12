use std::ffi::c_int;

/// A linux descriptor of any type
pub trait Descriptor {
    /// Get the raw underlying descriptor
    unsafe fn as_raw_handle(&self) -> c_int;
}

/// A [`Descriptor`] which can be made from a raw handle
pub trait FromRawHandle: Descriptor {
    /// Creates the descriptor from a file descriptor raw handle
    ///
    /// SAFETY: `handle` must be a valid file descriptor and it must represent this type
    unsafe fn from_raw_handle(handle: c_int) -> Self;
}
