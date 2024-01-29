use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, EBADF, EDQUOT, EINTR, EINVAL, EIO, ENOSPC, EROFS},
    unistd::{fsync, write},
};

extern "C" {
    /// [`fdatasync`] is similar to [`fsync`], but does not flush modified metadata unless that
    /// metadata is needed in order to allow a subsequent data retrieval to be correctly handled.
    /// For example, changes to `st_atime` or `st_mtime` (respectively, time of last access and
    /// time of last modification) do not require flushing because they are not necessary for a
    /// subsequent data read to be handled correctly. On the other hand, a change to the file size
    /// (`st_size`, as made by say [`ftruncate`]), would require a metadata flush.
    ///
    /// The aim of [`fdatasync`] is to reduce disk activity for applications that do not require
    /// all metadata to be synchronized with the disk.
    ///
    /// # Return Value
    /// On success, [`fdatasync`] returns zero. On error, -1 is returned, and [`errno`] is set to
    /// indicate the error.
    ///
    /// # Errors
    ///  * [`EBADF`] - `fd` is not a valid open file descriptor.
    ///  * [`EINTR`] - The function was interrupted by a signal.
    ///  * [`EIO`] - An error occurred during synchronization.  This error may relate to data
    ///              written to some other file descriptor on the same file. Since Linux 4.13,
    ///              errors from write-back will be reported to all file descriptors that might
    ///              have written the data which triggered the error. Some filesystems (e.g., NFS)
    ///              keep close track of which data came through which file descriptor, and give
    ///              more precise reporting. Other filesystems (e.g., most local filesystems) will
    ///              report errors to all file descriptors that were open on the file when the
    ///              error was recorded.
    ///  * [`ENOSPC`] - Disk space was exhausted while synchronizing.
    ///  * [`EROFS`] or [`EINVAL`] - `fd` is bound to a special file (e.g., a pipe, FIFO, or
    ///                              socket) which does not support synchronization.
    ///  * [`ENOSPC`] or [`EDQUOT`] - `fd` is bound to a file on NFS or another filesystem which
    ///                               does not allocate space at the time of a [`write()`] system
    ///                               call, and some previous write failed due to insufficient
    ///                               storage space.
    pub fn fdatasync(fd: c_int) -> c_int;
}
