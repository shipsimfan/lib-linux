use crate::aio::aiocb;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    aio::{aio_error, aio_return},
    errno::{errno, EAGAIN, EBADF, EFBIG, EINVAL, ENOSYS},
    fcntl::O_APPEND,
    unistd::write,
};

#[link(name = "rt")]
extern "C" {
    /// The [`aio_write`] function queues the I/O request described by the buffer pointed to by
    /// `aiocbp`. This function is the asynchronous analog of [`write()`]. The arguments of the
    /// call `write(fd, buf, count)` correspond (in order) to the fields `fildes`, `buf`, and
    /// `nbytes` of the structure pointed to by `aiocbp`.
    ///
    /// If [`O_APPEND`] is not set, the data is written starting at the absolute position
    /// `aiocbp.offset`, regardless of the file offset. If [`O_APPEND`] is set, data is written at
    /// the end of the file in the same order as [`aio_write`] calls are made. After the call, the
    /// value of the file offset is unspecified.
    ///
    /// The "asynchronous" means that this call returns as soon as the request has been enqueued;
    /// the write may or may not have completed when the call returns. One tests for completion
    /// using [`aio_error`]. The return status of a completed I/O operation can be obtained
    /// [`aio_return`]. Asynchronous notification of I/O completion can be obtained by setting
    /// `aiocbp.sigevent` appropriately.
    ///
    /// The field `aiocbp.lio_opcode` is ignored.
    ///
    /// No data is written to a regular file beyond its maximum offset.
    ///
    /// # Return Value
    /// On success, 0 is returned. On error, the request is not enqueued, -1 is returned, and
    /// [`errno`] is set to indicate the error. If an error is detected only later, it will be
    /// reported via [`aio_return`] (returns status -1) and [`aio_error`] (error status—whatever
    /// one would have gotten in [`errno`], such as [`EBADF`]).
    ///
    /// # Errors
    ///  * [`EAGAIN`] - Out of resources.
    ///  * [`EBADF`] - `fildes` is not a valid file descriptor open for writing.
    ///  * [`EFBIG`] - The file is a regular file, we want to write at least one byte, but the
    ///                starting position is at or beyond the maximum offset for this file.
    ///  * [`EINVAL`] - One or more of `offset`, `reqprio`, `nbytes` are invalid.
    ///  * [`ENOSYS`] - [`aio_write`] is not implemented.
    pub fn aio_write(aiocbp: *mut aiocb) -> c_int;
}
