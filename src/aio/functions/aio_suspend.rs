use crate::{aio::aiocb, time::timespec};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    aio::{aio_read, aio_write},
    errno::{errno, EAGAIN, EINTR, ENOSYS},
    time::CLOCK_MONOTONIC,
};
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "rt")]
extern "C" {
    /// The [`aio_suspend`] function suspends the calling thread until one of the following occurs:
    ///  * One or more of the asynchronous I/O requests in the list `aiocb_list` has completed.
    ///  * A signal is delivered.
    ///  * `timeout` is not [`null`] and the specified time interval has passed.
    ///
    /// The `nitems` argument specifies the number of items in `aiocb_list`. Each item in the list
    /// pointed to by `aiocb_list` must be either [`null`] (and then is ignored), or a pointer to a
    /// control block on which I/O was initiated using [`aio_read`], [`aio_write`], or
    /// [`lio_listio`].
    ///
    /// If [`CLOCK_MONOTONIC`] is supported, this clock is used to measure the timeout interval.
    ///
    /// # Return Value
    /// If this function returns after completion of one of the I/O requests specified in
    /// `aiocb_list`, 0 is returned. Otherwise, -1 is returned, and [`errno`] is set to indicate
    /// the error.
    ///
    /// # Errors
    ///  * [`EAGAIN`] - The call timed out before any of the indicated operations had completed.
    ///  * [`EINTR`] - The call was ended by signal (possibly the completion signal of one of the
    ///                operations we were waiting for).
    ///  * [`ENOSYS`] - [`aio_suspend`] is not implemented.
    pub fn aio_suspend(
        aiocb_list: *const *const aiocb,
        nitems: c_int,
        timeout: *const timespec,
    ) -> c_int;
}
