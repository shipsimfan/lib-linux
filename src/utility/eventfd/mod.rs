use std::ffi::c_int;

mod as_handle;
mod deref;
mod drop;
mod get;
mod new;
mod read;
mod write;

/// File descriptor for event notification
pub struct EventFd {
    /// The handle to the eventfd structure
    handle: c_int,
}
