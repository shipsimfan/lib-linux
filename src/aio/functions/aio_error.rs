use crate::aio::aiocb;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, ECANCELED, EINPROGRESS, EINVAL, ENOSYS},
    unistd::{read, write},
};

#[link(name = "rt")]
extern "C" {
    /// The [`aio_error`] function returns the error status for the asynchronous I/O request with
    /// control block pointed to by `aiocbp`.
    ///
    /// # Return Value
    /// This function returns one of the following:
    ///  * [`EINPROGRESS`] if the request has not been completed yet.
    ///  * [`ECANCELED`] if the request was canceled.
    ///  * `0` if the request completed successfully.
    ///  * A positive error number, if the asynchronous I/O operation failed. This is the same
    ///    value that would have been stored in the [`errno`] variable in the case of a synchronous
    ///    [`read`], [`write()`], [`fsync`], or [`fdatasync`] call.
    ///
    /// # Errors
    ///  * [`EINVAL`] - `aiocbp` does not point at a control block for an asynchronous I/O request
    ///                 of which the return status (see [`aio_return`]) has not been retrieved yet.
    ///  * [`ENOSYS`] - [`aio_error`] is not implemented.
    pub fn aio_error(aiocbp: *const aiocb) -> c_int;
}
