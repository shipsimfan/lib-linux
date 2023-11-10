use core::ffi::{c_size_t, c_void};

/// Structure for scatter/gather I/O
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct iovec {
    /// Pointer to data
    pub base: *mut c_void,

    /// Length of data
    pub len: c_size_t,
}
