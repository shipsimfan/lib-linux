use crate::aio::aiocb;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    aio::aio_error,
    errno::{errno, EAGAIN, EBADF, EINVAL, ENOSYS, EOVERFLOW},
    unistd::read,
};

#[link(name = "rt")]
extern "C" {
    /// The [`aio_read`] function queues the I/O request described by the buffer pointed to by
    /// `aiocbp`. This function is the asynchronous analog of [`read`]. The arguments of the call
    /// `read(fd, buf, count)` correspond (in order) to the fields `filedes`, `buf`, and `nbytes`
    /// of the structure pointed to by `aiocbp`.
    ///
    /// The data is read starting at the absolute position `aiocbp.offset`, regardless of the file
    /// offset. After the call, the value of the file offset is unspecified.
    ///
    /// The "asynchronous" means that this call returns as soon as the request has been enqueued;
    /// the read may or may not have completed when the call returns. One tests for completion
    /// using [`aio_error`]. The return status of a completed I/O operation can be obtained by
    /// [`aio_return`]. Asynchronous notification of I/O completion can be obtained by setting
    /// `aiocbp.aio_sigevent` appropriately.
    ///
    /// The field `aiocbp.lio_opcode` is ignored.
    ///
    /// No data is read from a regular file beyond its maximum offset.
    ///
    /// # Return Value
    /// On success, 0 is returned. On error, the request is not enqueued, -1 is returned, and
    /// [`errno`] is set to indicate the error. If an error is detected only later, it will be
    /// reported via [`aio_return`] (returns status -1) and [`aio_error`] (error status—whatever
    /// one would have gotten in [`errno`], such as [`EBADF`]).
    ///
    /// # Errors
    ///  * [`EAGAIN`] - Out of resources
    ///  * [`EBADF`] - `fildes` is not a valid file descriptor open for reading
    ///  * [`EINVAL`] - One or more of `offset`, `reqprio`, or `nbytes` are invalid
    ///  * [`ENOSYS`] - [`aio_read`] is not implemented
    ///  * [`EOVERFLOW`] - The file is a regular file, we start reading before end-of-file and want
    ///                    at least one byte, but the starting position is past the maximum offset
    ///                    for this file.
    pub fn aio_read(aiocbp: *mut aiocb) -> c_int;
}
