use std::ffi::c_int;

/// Add a file descriptor to the interface.
pub const EPOLL_CTL_ADD: c_int = 1;

/// Remove a file descriptor from the interface.
pub const EPOLL_CTL_DEL: c_int = 2;

/// Change file descriptor epoll_event structure.
pub const EPOLL_CTL_MOD: c_int = 3;
