use std::ffi::c_int;

/// Close the epoll on [`exec`]
pub const EPOLL_CLOEXEC: c_int = 02000000;
