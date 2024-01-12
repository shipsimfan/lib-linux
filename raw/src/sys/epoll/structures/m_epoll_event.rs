use std::ffi::{c_int, c_void};

/// The object linked to a file descriptor
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct epoll_event {
    /// Epoll events
    pub events: u32,

    /// User data variable
    pub data: epoll_data_t,
}

/// The data associated with an object linked to a file descriptor
#[repr(C)]
#[allow(non_camel_case_types)]
pub union epoll_data_t {
    /// As a pointer
    pub ptr: *mut c_void,

    /// As a file descriptor
    pub fd: c_int,

    /// As a [`u32`]
    pub u32: u32,

    /// As a [`u64`]
    pub u64: u64,
}