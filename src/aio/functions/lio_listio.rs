use crate::{aio::aiocb, signal::sigevent};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    aio::{
        aio_error, aio_read, aio_return, aio_write, LIO_NOP, LIO_NOWAIT, LIO_READ, LIO_WAIT,
        LIO_WRITE,
    },
    errno::{errno, EAGAIN, EINTR, EINVAL, EIO},
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "rt")]
extern "C" {
    /// The [`lio_listio`] function initiates the list of I/O operations described by the array
    /// `aiocb_list`.
    ///
    /// The `mode` operation has one of the following values:
    ///  * [`LIO_WAIT`] - The call blocks until all operations are complete. The `sevp` argument is
    ///                   ignored.
    ///  * [`LIO_NOWAIT`] - The I/O operations are queued for processing and the call returns
    ///                     immediately. When all of the I/O operations complete, asynchronous
    ///                     notification occurs, as specified by the `sevp` argument. If `sevp` is
    ///                     [`null_mut`], no asynchronous notification occurs.
    ///
    /// The `aiocb_list` argument is an array of pointers to aiocb structures that describe I/O
    /// operations. These operations are executed in an unspecified order. The `nitems` argument
    /// specifies the size of the array `aiocb_list`. [`null_mut`] pointers in `aiocb_list` are
    /// ignored.
    ///
    /// In each control block in `aiocb_list`, the `lio_opcode` field specifies the I/O operation
    /// to be initiated, as follows:
    ///  * [`LIO_READ`] - Initiate a read operation. The operation is queued as for a call to
    ///                   [`aio_read`] specifying this control block.
    ///  * [`LIO_WRITE`] - Initiate a write operation. The operation is queued as for a call to
    ///                    [`aio_write`] specifying this control block.
    ///  * [`LIO_NOP`] - Ignore this control block.
    ///
    /// The remaining fields in each control block have the same meanings as for [`aio_read`] and
    /// [`aio_write`]. The `sigevent` fields of each control block can be used to specify
    /// notifications for the individual I/O operations.
    ///
    /// # Return Value
    /// If `mode` is [`LIO_NOWAIT`], [`lio_listio`] returns 0 if all I/O operations are
    /// successfully queued. Otherwise, -1 is returned, and [`errno`] is set to indicate the error.
    ///
    /// If `mode` is [`LIO_WAIT`], [`lio_listio`] returns 0 when all of the I/O operations have
    /// completed successfully. Otherwise, -1 is returned, and [`errno`] is set to indicate the
    /// error.
    ///
    /// The return status from [`lio_listio`] provides information only about the call itself, not
    /// about the individual I/O operations. One or more of the I/O operations may fail, but this
    /// does not prevent other operations completing. The status of individual I/O operations in
    /// `aiocb_list` can be determined using [`aio_error`]. When an operation has completed, its
    /// return status can be obtained using [`aio_return`]. Individual I/O operations can fail for
    /// the reasons described in [`aio_read`] and [`aio_write`].
    ///
    /// # Errors
    /// The [`lio_listio`] function may fail for the following reasons:
    ///  * [`EAGAIN`] - Out of resources.
    ///  * [`EAGAIN`] - The number of I/O operations specified by `nitems` would cause the limit
    ///                 [`AIO_MAX`] to be exceeded.
    ///  * [`EINTR`] - `mode` was [`LIO_WAIT`] and a signal was caught before all I/O operations
    ///                 completed. (This may even be one of the signals used for asynchronous I/O
    ///                 completion notification.)
    ///  * [`EINVAL`] - `mode` is invalid, or `nitems` exceeds the limit [`AIO_LISTIO_MAX`].
    ///  * [`EIO`] - One of more of the operations specified by `aiocb_list` failed. The
    ///              application can check the status of each operation using [`aio_return`].
    ///
    /// If [`lio_listio`] fails with the error [`EAGAIN`], [`EINTR`], or [`EIO`], then some of the
    /// operations in `aiocb_list` may have been initiated. If [`lio_listio`] fails for any other
    /// reason, then none of the I/O operations has been initiated.
    pub fn lio_listio(
        mode: c_int,
        aiocb_list: *const *mut aiocb,
        nitems: c_int,
        sevp: *mut sigevent,
    ) -> c_int;
}
