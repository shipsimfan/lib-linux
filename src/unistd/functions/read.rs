use crate::c_ssize_t;
use std::ffi::{c_int, c_void};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, EAGAIN, EBADF, EFAULT, EINTR, EINVAL, EIO, EISDIR, EWOULDBLOCK},
    fcntl::{open, O_DIRECT, O_NONBLOCK},
    signal::SIGTTIN,
    sys::timerfd::timerfd_create,
};

extern "C" {
    /// [`read`] attempts to read up to `count` bytes from file descriptor `fd` into the buffer
    /// starting at `buf`.
    ///
    /// On files that support seeking, the read operation commences at the file offset, and the
    /// file offset is incremented by the number of bytes read. If the file offset is at or past
    /// the end of file, no bytes are read, and [`read`] returns zero.
    ///
    /// If `count` is zero, [`read`] may detect the errors described below. In the absence of any
    /// errors, or if [`read`] does not check for errors, a [`read`] with a count of 0 returns zero
    /// and has no other effects.
    ///
    /// According to POSIX.1, if `count` is greater than [`c_ssize_t::MAX`], the result is
    /// implementation-defined.
    ///
    /// # Return Value
    /// On success, the number of bytes read is returned (zero indicates end of file), and the file
    /// position is advanced by this number. It is not an error if this number is smaller than the
    /// number of bytes requested; this may happen for example because fewer bytes are actually
    /// available right now (maybe because we were close to end-of-file, or because we are reading
    /// from a pipe, or from a terminal), or because [`read`] was interrupted by a signal.
    ///
    /// On error, -1 is returned, and [`errno`] is set to indicate the error. In this case, it is
    /// left unspecified whether the file position (if any) changes.
    ///
    /// # Errors    
    ///  * [`EAGAIN`] - The file descriptor `fd` refers to a file other than a socket and has been
    ///                 marked nonblocking ([`O_NONBLOCK`]), and the read would block. See [`open`]
    ///                 for further details on the [`O_NONBLOCK`] flag.
    ///  * [`EAGAIN`] or [`EWOULDBLOCK`] - The file descriptor fd refers to a socket and has been
    ///                                    marked nonblocking ([`O_NONBLOCK`]), and the read would
    ///                                    block. POSIX.1-2001 allows either error to be returned
    ///                                    for this case, and does not require these constants to
    ///                                    have the same value, so a portable application should
    ///                                    check for both possibilities.
    ///  * [`EBADF`] - `fd` is not a valid file descriptor or is not open for reading.
    ///  * [`EFAULT`] - `buf` is outside your accessible address space.
    ///  * [`EINTR`] - The call was interrupted by a signal before any data was read.
    ///  * [`EINVAL`] - `fd` is attached to an object which is unsuitable for reading; or the file
    ///                 was opened with the [`O_DIRECT`] flag, and either the address specified in
    ///                 `buf`, the value specified in `count`, or the file offset is not suitably
    ///                 aligned.
    ///  * [`EINVAL`] - `fd` was created via a call to [`timerfd_create`] and the wrong size buffer
    ///                 was given to [`read`]; see [`timerfd_create`] for further information.
    ///  * [`EIO`] - I/O error. This will happen for example when the process is in a background
    ///              process group, tries to read from its controlling terminal, and either it is
    ///              ignoring or blocking [`SIGTTIN`] or its process group is orphaned. It may also
    ///              occur when there is a low-level I/O error while reading from a disk or tape. A
    ///              further possible cause of [`EIO`] on networked filesystems is when an advisory
    ///              lock had been taken out on the file descriptor and this lock has been lost.
    ///  * [`EISDIR`] - `fd` refers to a directory.
    ///
    /// Other errors may occur, depending on the object connected to `fd`.
    pub fn read(fd: c_int, buf: *mut c_void, count: c_ssize_t) -> c_ssize_t;
}
