use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, EBADF, EDQUOT, EINTR, EIO, ENOSPC},
    fcntl::open,
    unistd::{fsync, write},
};

extern "C" {
    /// [`close`] closes a file descriptor, so that it no longer refers to any file and may be
    /// reused. Any record locks (see [`fcntl`]) held on the file it was associated with, and owned
    /// by the process, are removed (regardless of the file descriptor that was used to obtain the
    /// lock).
    ///
    /// If `fd` is the last file descriptor referring to the underlying open file description (see
    /// [`open`]), the resources associated with the open file description are freed; if the file
    /// descriptor was the last reference to a file which has been removed using [`unlink`], the
    /// file is deleted.
    ///
    /// # Return Value
    /// [`close`] returns zero on success. On error, -1 is returned, and [`errno`] is set to
    /// indicate the error.
    ///
    /// # Errors
    ///  * [`EBADF`] - `fd` isn't a valid open file descriptor.
    ///  * [`EINTR`] - The [`close`] call was interrupted by a signal.
    ///  * [`EIO`] - An I/O error occurred.
    ///  * [`ENOSPC`], [`EDQUOT`] - On NFS, these errors are not normally reported against the
    ///                             first write which exceeds the available storage space, but
    ///                             instead against a subsequent [`write()`], [`fsync`], or
    ///                             [`close`].
    pub fn close(fd: c_int) -> c_int;
}
