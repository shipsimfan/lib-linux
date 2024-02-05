use std::ffi::{c_int, c_void};

/// The [`epoll_event`] structure specifies data that the kernel should save and return when the
/// corresponding file descriptor becomes ready.
#[repr(packed(4))]
#[allow(non_camel_case_types)]
pub struct epoll_event {
    /// Epoll events
    pub events: u32,

    /// User data variable
    pub data: epoll_data_t,
}

/// The data associated with an object linked to a file descriptor
#[repr(packed(4))]
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

impl Default for epoll_event {
    fn default() -> Self {
        epoll_event {
            events: 0,
            data: epoll_data_t::default(),
        }
    }
}

impl Default for epoll_data_t {
    fn default() -> Self {
        epoll_data_t { u64: 0 }
    }
}
