use std::ffi::c_int;

/// Only send the event this this epoll if multiple are waiting
pub const EPOLLEXCLUSIVE: c_int = 1 << 28;

/// Prevent the OS from entering "suspend" or "hibernate"
pub const EPOLLWAKEUP: c_int = 1 << 29;

/// One-shot notifications
pub const EPOLLONESHOT: c_int = 1 << 30;

/// Edge-triggered notifications
pub const EPOLLET: c_int = 1 << 31;
