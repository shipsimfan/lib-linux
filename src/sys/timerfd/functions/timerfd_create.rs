use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, EINVAL, EMFILE, ENFILE, ENODEV, ENOMEM},
    fcntl::{open, O_CLOEXEC, O_NONBLOCK},
    sys::timerfd::{TFD_CLOEXEC, TFD_NONBLOCK},
    time::{CLOCK_MONOTONIC, CLOCK_REALTIME},
};

extern "C" {
    /// [`timerfd_create`] creates a new timer object, and returns a file descriptor that refers to
    /// that timer. The `clockid` argument specifies the clock that is used to mark the progress of
    /// the timer, and must be either [`CLOCK_REALTIME`] or [`CLOCK_MONOTONIC`]. [`CLOCK_REALTIME`]
    /// is a settable system-wide clock. [`CLOCK_MONOTONIC`] is a nonsettable clock that is not
    /// affected by discontinuous changes in the system clock (e.g., manual changes to system
    /// time). The current value of each of these clocks can be retrieved using [`clock_gettime`].
    ///
    /// Starting with Linux 2.6.27, the following values may be bitwise ORed in flags to change the
    /// behavior of [`timerfd_create`]:
    ///  * [`TFD_NONBLOCK`] - Set the [`O_NONBLOCK`] file status flag on the new open file
    ///                       description. Using this flag saves extra calls to [`fcntl`] to
    ///                       achieve the same result.
    ///  * [`TFD_CLOEXEC`] - Set the close-on-exec ([`FD_CLOEXEC`]) flag on the new file
    ///                      descriptor. See the description of the [`O_CLOEXEC`] flag in [`open`]
    ///                      for reasons why this may be useful.
    ///
    /// In Linux versions up to and including 2.6.26, flags must be specified as zero.
    ///
    /// # Return Value
    /// On success, [`timerfd_create`] returns a new file descriptor. On error, -1 is returned and
    /// [`errno`] is set to indicate the error.
    ///
    /// # Errors
    /// [`timerfd_create`] can fail with the following errors:
    ///  * [`EINVAL`] - The `clockid` argument is neither [`CLOCK_MONOTONIC`] nor
    ///                 [`CLOCK_REALTIME`];
    ///  * [`EINVAL`] - `flags` is invalid; or, in Linux 2.6.26 or earlier, `flags` is nonzero.
    ///  * [`EMFILE`] - The per-process limit of open file descriptors has been reached.
    ///  * [`ENFILE`] - The system-wide limit on the total number of open files has been reached.
    ///  * [`ENODEV`] - Could not mount (internal) anonymous inode device.
    ///  * [`ENOMEM`] - There was insufficient kernel memory to create the timer.
    pub fn timerfd_create(clockid: c_int, flags: c_int) -> c_int;
}
