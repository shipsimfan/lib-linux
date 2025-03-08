use crate::poll::{nfds_t, pollfd};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::sys::epoll;

#[link(name = "c")]
extern "system" {
    /// Wait for some event on a file descriptor
    ///
    /// # Description
    /// [`poll`] performs a similar task to [`select`]: it waits for one of a set of file
    /// descriptors to become ready to perform I/O. The Linux-specific [`epoll`] API performs a
    /// similar task, but offers features beyond those found in [`poll`].
    ///
    /// The set of file descriptors to be monitored is specified in the `fds` argument, which is an
    /// array of [`pollfd`] structures.
    ///
    /// The caller should specify the number of items in the `fds` array in `nfds`.
    ///
    /// The field `fd` contains a file descriptor for an open file. If this field is negative, then
    /// the corresponding events field is ignored and the revents field returns zero. (This
    /// provides an easy way of ignoring a file descriptor for a single [`poll`] call: simply set
    /// the `fd` field to its bitwise complement.)
    ///
    /// The field `events` is an input parameter, a bit mask specifying the events the application
    /// is interested in for the file descriptor `fd`. This field may be specified as zero, in
    /// which case the only events that can be returned in revents are [`POLLHUP`], [`POLLERR`],
    /// and [`POLLNVAL`].
    ///
    /// The field revents is an output parameter, filled by the kernel with the events that
    /// actually occurred. The bits returned in revents can include any of those specified in
    /// events, or one of the values [`POLLERR`], [`POLLHUP`], or [`POLLNVAL`]. (These three bits
    /// are meaningless in the events field, and will be set in the revents field whenever the
    /// corresponding condition is true.)
    ///
    /// If none of the events requested (and no error) has occurred for any of the file
    /// descriptors, then [`poll`] blocks until one of the events occurs.
    ///
    /// The timeout argument specifies the number of milliseconds that [`poll`] should block
    /// waiting for a file descriptor to become ready. The call will block until either:
    ///  - a file descriptor becomes ready;
    ///  - the call is interrupted by a signal handler; or
    ///  - the timeout expires.
    ///
    /// Being "ready" means that the requested operation will not block; thus, [`poll`]ing regular
    /// files, block devices, and other files with no reasonable polling semantic always returns
    /// instantly as ready to read and write.
    ///
    /// Note that the timeout interval will be rounded up to the system clock granularity, and
    /// kernel scheduling delays mean that the blocking interval may overrun by a small amount.
    /// Specifying a negative value in timeout means an infinite timeout. Specifying a timeout of
    /// zero causes [`poll`] to return immediately, even if no file descriptors are ready.
    pub fn poll(fds: *mut pollfd, nfds: nfds_t, timeout: c_int) -> c_int;
}
