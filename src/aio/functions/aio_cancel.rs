use crate::aio::aiocb;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    aio::{aio_error, aio_return, AIO_ALLDONE, AIO_CANCELED, AIO_NOTCANCELED},
    errno::{errno, EBADF, ECANCELED, EINPROGRESS, ENOSYS},
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "rt")]
extern "C" {
    /// The [`aio_cancel`] function attempts to cancel outstanding asynchronous I/O requests for
    /// the file descriptor `fd`. If `aiocbp` is [`null_mut`], all such requests are canceled.
    /// Otherwise, only the request described by the control block pointed to by `aiocbp` is
    /// canceled.
    ///
    /// Normal asynchronous notification occurs for canceled request. The request return status
    /// ([`aio_return`]) is set to -1, and the request error status ([`aio_error`]) is set to
    /// [`ECANCELED`]. The control block of requests that cannot be canceled is not changed.
    ///
    /// If the request could not be canceled, then it will terminate in the usual way after
    /// performing the I/O operation. (In this case, [`aio_error`] will return the status
    /// [`EINPROGRESS`].)
    ///
    /// If `aiocbp` is not [`null_mut`], and `fd` differs from the file descriptor with which the
    /// asynchronous operation was initiated, unspecified results occur.
    ///
    /// Which operations are cancelable is implementation-defined.
    ///
    /// # Return Value
    /// The [`aio_cancel`] function returns one of the following values:
    ///  * [`AIO_CANCELED`] - All requests were successfully canceled.
    ///  * [`AIO_NOTCANCELED`] - At least one of the requests specified was not canceled because it
    ///                          was in progress. In this case, one may check the status of
    ///                          individual requests using [`aio_error`].
    ///  * [`AIO_ALLDONE`] - All requests had already been completed before the call.
    ///  * `-1` - An error occurred. The cause of the error can be found by inspecting [`errno`].
    ///
    /// # Errors
    ///  * [`EBADF`] - `fd` is not a valid file descriptor.
    ///  * [`ENOSYS`] - [`aio_cancel`] is not implemented.
    pub fn aio_cancel(fd: c_int, aiocbp: *mut aiocb) -> c_int;
}
