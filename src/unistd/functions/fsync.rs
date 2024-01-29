use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, EBADF, EDQUOT, EINTR, EINVAL, EIO, ENOSPC, EROFS},
    unistd::write,
};

extern "C" {
    /// [`fsync`] transfers ("flushes") all modified in-core data of (i.e., modified buffer cache
    /// pages for) the file referred to by the file descriptor `fd` to the disk device (or other
    /// permanent storage device) so that all changed information can be retrieved even if the
    /// system crashes or is rebooted. This includes writing through or flushing a disk cache if
    /// present. The call blocks until the device reports that the transfer has completed.
    ///
    /// As well as flushing the file data, [`fsync`] also flushes the metadata information
    /// associated with the file.
    ///
    /// Calling [`fsync`] does not necessarily ensure that the entry in the directory containing
    /// the file has also reached disk. For that an explicit [`fsync`] on a file descriptor for the
    /// directory is also needed.
    ///
    /// # Return Value
    /// On success, [`fsync`] returns zero. On error, -1 is returned, and [`errno`] is set to
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
    pub fn fsync(fd: c_int) -> c_int;
}
