use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, EINVAL, EMFILE, ENFILE, ENOMEM},
    unistd::close,
};

extern "C" {
    /// [`epoll_create`] creates a new epoll instance. Since Linux 2.6.8, the size argument is
    /// ignored, but must be greater than zero.
    ///
    /// [`epoll_create`] returns a file descriptor referring to the new epoll instance. This file
    /// descriptor is used for all the subsequent calls to the epoll interface. When no longer
    /// required, the file descriptor returned by [`epoll_create`] should be closed by using
    /// [`close`]. When all file descriptors referring to an epoll instance have been closed, the
    /// kernel destroys the instance and releases the associated resources for reuse.
    ///
    /// # Return Value
    /// On success, [`epoll_create`] returns a file descriptor (a nonnegative integer). On error,
    /// -1 is returned, and [`errno`] is set to indicate the error.
    ///
    /// # Errors
    ///  * [`EINVAL`] - `size` is not positive.
    ///  * [`EMFILE`] - The per-process limit on the number of open file descriptors has been
    ///                 reached.
    ///  * [`ENFILE`] - The system-wide limit on the total number of open file descriptors has been
    ///                 reached.
    ///  * [`ENOMEM`] - There was insufficient memory to create the kernel object.
    pub fn epoll_create(size: c_int) -> c_int;
}
