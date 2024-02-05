use std::ffi::{c_int, c_uint};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, EINVAL, EMFILE, ENFILE, ENODEV, ENOMEM},
    fcntl::{open, O_CLOEXEC, O_NONBLOCK},
};

extern "C" {
    /// [`eventfd`] creates an "eventfd object" that can be used as an event wait/notify mechanism
    /// by user-space applications, and by the kernel to notify user-space applications of events.
    /// The object contains an unsigned 64-bit integer ([`u64`]) counter that is maintained by the
    /// kernel. This counter is initialized with the value specified in the argument `initval`.
    ///
    /// As its return value, [`eventfd`] returns a new file descriptor that can be used to refer to
    /// the eventfd object.
    ///
    /// The following values may be bitwise ORed in `flags` to change the behavior of [`eventfd`]:
    ///  * [`EFD_CLOEXEC`] - (since Linux 2.6.27) Set the close-on-exec ([`FD_CLOEXEC`]) flag on
    ///                      the new file descriptor. See the description of the [`O_CLOEXEC`] flag
    ///                      in [`open`] for reasons why this may be useful.
    ///  * [`EFD_NONBLOCK`] - (since Linux 2.6.27) Set the [`O_NONBLOCK`] file status flag on the
    ///                       open file description (see [`open`]) referred to by the new file
    ///                       descriptor. Using this flag saves extra calls to [`fcntl`] to achieve
    ///                       the same result.
    ///  * [`EFD_SEMAPHORE`] - (since Linux 2.6.30) Provide semaphore-like semantics for reads from
    ///                        the new file descriptor.
    ///
    /// # Return Value
    /// On success, [`eventfd`] returns a new eventfd file descriptor. On error, -1 is returned and
    /// [`errno`] is set to indicate the error.
    ///
    /// # Errors
    ///  * [`EINVAL`] - An unsupported value was specified in `flags`.
    ///  * [`EMFILE`] - The per-process limit on the number of open file descriptors has been
    ///                 reached.
    ///  * [`ENFILE`] - The system-wide limit on the total number of open files has been reached.
    ///  * [`ENODEV`] - Could not mount (internal) anonymous inode device.
    ///  * [`ENOMEM`] - There was insufficient memory to create a new eventfd file descriptor.
    pub fn eventfd(initval: c_uint, flags: c_int) -> c_int;
}
