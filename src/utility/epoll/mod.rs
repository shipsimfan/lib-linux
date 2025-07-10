use std::ffi::c_int;

mod as_handle;
mod ctl;
mod deref;
mod drop;
mod get;
mod new;
mod wait;

/// I/O event notification facility
pub struct EPoll {
    /// The handle to the epoll structure
    handle: c_int,
}
