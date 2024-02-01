use crate::signal::sigset_t;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, EBADF, EINVAL, EMFILE, ENFILE, ENODEV, ENOMEM},
    fcntl::{open, O_CLOEXEC, O_NONBLOCK},
    signal::{SIGKILL, SIGSTOP},
    sys::{
        epoll,
        signalfd::{SFD_CLOEXEC, SFD_NONBLOCK},
    },
};

extern "C" {
    /// [`signalfd`] creates a file descriptor that can be used to accept signals targeted at the
    /// caller. This provides an alternative to the use of a signal handler or [`sigwaitinfo`], and
    /// has the advantage that the file descriptor may be monitored by [`select`], [`poll`], and
    /// [`epoll`].
    ///
    /// The mask argument specifies the set of signals that the caller wishes to accept via the
    /// file descriptor. This argument is a signal set whose contents can be initialized using the
    /// macros described in [`sigsetops`]. Normally, the set of signals to be received via the file
    /// descriptor should be blocked using [`sigprocmask`], to prevent the signals being handled
    /// according to their default dispositions. It is not possible to receive [`SIGKILL`] or
    /// [`SIGSTOP`] signals via a signalfd file descriptor; these signals are silently ignored if
    /// specified in mask.
    ///
    /// If the `fd` argument is -1, then the call creates a new file descriptor and associates the
    /// signal set specified in mask with that file descriptor. If `fd` is not -1, then it must
    /// specify a valid existing signalfd file descriptor, and mask is used to replace the signal
    /// set associated with that file descriptor.
    ///
    /// Starting with Linux 2.6.27, the following values may be bitwise ORed in flags to change the
    /// behavior of [`signalfd`]:
    ///  * [`SFD_NONBLOCK`] - Set the [`O_NONBLOCK`] file status flag on the open file description
    ///                       (see [`open`]) referred to by the new file descriptor. Using this
    ///                       flag saves extra calls to [`fcntl`] to achieve the same result.
    ///  * [`SFD_CLOEXEC`] - Set the close-on-exec ([`FD_CLOEXEC`]) flag on the new file
    ///                      descriptor. See the description of the [`O_CLOEXEC`] flag in [`open`]
    ///                      for reasons why this may be useful.
    ///
    /// # Return Value
    /// On success, [`signalfd`] returns a signalfd file descriptor; this is either a new file
    /// descriptor (if `fd` was -1), or `fd` if `fd` was a valid signalfd file descriptor. On
    /// error, -1 is returned and [`errno`] is set to indicate the error.
    ///
    /// # Errors
    ///  * [`EBADF`] - The `fd` file descriptor is not a valid file descriptor.
    ///  * [`EINVAL`] - `fd` is not a valid signalfd file descriptor.
    ///  * [`EINVAL`] - flags is invalid; or, in Linux 2.6.26 or earlier, flags is nonzero.
    ///  * [`EMFILE`] - The per-process limit on the number of open file descriptors has been
    ///                 reached.
    ///  * [`ENFILE`] - The system-wide limit on the total number of open files has been reached.
    ///  * [`ENODEV`] - Could not mount (internal) anonymous inode device.
    ///  * [`ENOMEM`] - There was insufficient memory to create a new signalfd file descriptor.
    pub fn signalfd(fd: c_int, mask: *const sigset_t, flags: c_int) -> c_int;
}
