use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{errno::errno, sys::epoll::epoll_create, unistd::close};

extern "C" {
    /// epoll_create1 - open an epoll file descriptor
    ///
    /// # Description
    /// [`epoll_create1`] creates a new epoll instance. If flags is 0, then, other than the fact
    /// that the obsolete size argument is dropped, [`epoll_create1`] is the same as
    /// [`epoll_create`].
    ///
    /// [`epoll_create1`] returns a file descriptor referring to the new epoll instance. This file
    /// descriptor is used for all the subsequent calls to the epoll interface. When no longer
    /// required, the file descriptor returned by [`epoll_create1`] should be closed by using
    /// [`close`]. When all file descriptors referring to an epoll instance have been closed, the
    /// kernel destroys the instance and releases the associated resources for reuse.
    ///
    /// # Return Value
    /// On success, these system calls return a file descriptor (a nonnegative integer).  On error,
    /// -1 is returned, and [`errno`] is set to indicate the error
    pub fn epoll_create1(flags: c_int) -> c_int;
}
