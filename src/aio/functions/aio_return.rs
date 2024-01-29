use crate::aio::aiocb;
use core::ffi::c_ssize_t;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    aio::aio_error,
    errno::{errno, EINPROGRESS},
    unistd::{fdatasync, fsync, read, write},
};

#[link(name = "rt")]
extern "C" {
    /// The [`aio_return`] function returns the final return status for the asynchronous I/O
    /// request with control block pointed to by `aiocbp`.
    ///
    /// This function should be called only once for any given request, after [`aio_error`] returns
    /// something other than [`EINPROGRESS`].
    ///
    /// # Return Value
    /// If the asynchronous I/O operation has completed, this function returns the value that would
    /// have been returned in case of a synchronous [`read`], [`write()`], [`fsync`], or
    /// [`fdatasync`], call. On error, -1 is returned, and [`errno`] is set to indicate the error.
    ///
    /// If the asynchronous I/O operation has not yet completed, the return value and effect of
    /// [`aio_return`] are undefined.
    ///
    /// # Errors
    ///  * [`EINVAL`] - `aiocbp` does not point at a control block for an asynchronous I/O request
    ///                 of which the return status has not been retrieved yet.
    ///  * [`ENOSYS`] - [`aio_return`] is not implemented.
    pub fn aio_return(aiocbp: *mut aiocb) -> c_ssize_t;
}
