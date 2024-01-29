use crate::aio::aiocb;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, EAGAIN, EBADF, EINVAL, ENOSYS},
    fcntl::{O_DSYNC, O_SYNC},
    signal::sigevent,
    unistd::{fdatasync, fsync},
};

#[link(name = "rt")]
extern "C" {
    /// The [`aio_fsync`] function does a sync on all outstanding asynchronous I/O operations
    /// associated with `aiocbp.fildes`.
    ///
    /// More precisely, if `op` is [`O_SYNC`], then all currently queued I/O operations shall be
    /// completed as if by a call of [`fsync`], and if `op` is [`O_DSYNC`], this call is the
    /// asynchronous analog of [`fdatasync`].
    ///
    /// Note that this is a request only; it does not wait for I/O completion.
    ///
    /// Apart from `fildes`, the only field in the structure pointed to by `aiocbp` that is used by
    /// this call is the `sigevent` field (a [`sigevent`] structure), which indicates the desired
    /// type of asynchronous notification at completion. All other fields are ignored.
    ///
    /// # Return Value
    /// On success (the sync request was successfully queued) this function returns 0. On error, -1
    /// is returned, and [`errno`] is set to indicate the error.
    ///
    /// # Errors
    ///  * [`EAGAIN`] - Out of resources.
    ///  * [`EBADF`] - `fildes` is not a valid file descriptor open for writing.
    ///  * [`EINVAL`] - Synchronized I/O is not supported for this file, or `op` is not [`O_SYNC`]
    ///                 or [`O_DSYNC`].
    ///  * [`ENOSYS`] - [`aio_fsync`] is not implemented.
    pub fn aio_fsync(op: c_int, aiocbp: *mut aiocb) -> c_int;
}
