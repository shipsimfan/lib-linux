use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, EINVAL, EMFILE, ENFILE, ENOMEM},
    sys::epoll::{epoll_create, EPOLL_CLOEXEC},
    unistd::close,
};

extern "C" {
    /// epoll_create1 - open an epoll file descriptor
    ///
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
    /// If `flags` is 0, then, other than the fact that the obsolete `size` argument is dropped,
    /// [`epoll_create1`] is the same as [`epoll_create`]. The following value can be included in
    /// flags to obtain different behavior:
    ///  * [`EPOLL_CLOEXEC`] - Set the close-on-exec ([`FD_CLOEXEC`]) flag on the new file
    ///                        descriptor. See the description of the [`O_CLOEXEC`] flag in
    ///                        [`open`] for reasons why this may be useful.
    ///
    /// # Return Value
    /// On success, these system calls return a file descriptor (a nonnegative integer).  On error,
    /// -1 is returned, and [`errno`] is set to indicate the error
    ///
    /// # Errors
    ///  * [`EINVAL`] - Invalid value specified in `flags`.
    ///  * [`EMFILE`] - The per-process limit on the number of open file descriptors has been
    ///                 reached.
    ///  * [`ENFILE`] - The system-wide limit on the total number of open file descriptors has been
    ///                 reached.
    ///  * [`ENOMEM`] - There was insufficient memory to create the kernel object.
    pub fn epoll_create1(flags: c_int) -> c_int;
}
