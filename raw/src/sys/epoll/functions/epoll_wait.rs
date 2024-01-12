use crate::sys::epoll::epoll_event;
use std::ffi::c_int;

extern "C" {
    /// epoll_wait - wait for an I/O event on an epoll file descriptor
    ///
    /// The [`epoll_wait`] system call waits for events on the epoll instance referred to by the
    /// file descriptor `epfd`. The buffer pointed to by events is used to return information from
    /// the ready list about file descriptors in the interest list that have some events available.
    /// Up to `maxevents` are returned by [`epoll_wait`]. The `maxevents` argument must be greater
    /// than zero.
    ///
    /// The `timeout` argument specifies the number of milliseconds that [`epoll_wait`] will block.
    /// Time is measured against the [`CLOCK_MONOTONIC`] clock.
    ///
    /// A call to [`epoll_wait`] will block until either:
    /// - a file descriptor delivers an event;
    /// - the call is interrupted by a signal handler; or
    /// - the timeout expires.
    ///
    /// Note that the timeout interval will be rounded up to the system clock granularity, and
    /// kernel scheduling delays mean that the blocking interval may overrun by a small amount.
    /// Specifying a timeout of -1 causes [`epoll_wait`] to block indefinitely, while specifying
    /// a timeout equal to zero cause [`epoll_wait`] to return immediately, even if no events are
    /// available.
    ///
    /// The data field of each returned [`epoll_event`] structure contains the same data as was
    /// specified in the most recent call to [`epoll_ctl`] ([`EPOLL_CTL_ADD`], [`EPOLL_CTL_MOD`])
    /// for the corresponding open file descriptor.
    ///
    /// The events field is a bit mask that indicates the events that have occurred for the
    /// corresponding open file description. See [`epoll_ctl`] for a list of the bits that may
    /// appear in this mask.
    ///
    /// # Return Value
    pub fn epoll_wait(
        epfd: c_int,
        events: *mut epoll_event,
        max_events: c_int,
        timeout: c_int,
    ) -> c_int;
}
