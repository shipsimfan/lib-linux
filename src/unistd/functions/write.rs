use core::ffi::c_ssize_t;
use std::ffi::{c_int, c_void};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{
        errno, EAGAIN, EBADF, EDESTADDRREQ, EDQUOT, EFAULT, EFBIG, EINTR, EINVAL, EIO, ENOSPC,
        EPERM, EPIPE, EWOULDBLOCK,
    },
    fcntl::{open, O_APPEND, O_DIRECT, O_NONBLOCK},
    unistd::read,
};

extern "C" {
    /// [`write()`] writes up to `count` bytes from the buffer starting at `buf` to the file referred
    /// to by the file descriptor `fd`.
    ///
    /// The number of bytes written may be less than `count` if, for example, there is insufficient
    /// space on the underlying physical medium, or the [`RLIMIT_FSIZE`] resource limit is
    /// encountered (see [`setrlimit`]), or the call was interrupted by a signal handler after
    /// having written less than `count` bytes. (See also [`pipe`].)
    ///
    /// For a seekable file (i.e., one to which [`lseek`] may be applied, for example, a regular
    /// file) writing takes place at the file offset, and the file offset is incremented by the
    /// number of bytes actually written. If the file was [`open`]ed with [`O_APPEND`], the file
    /// offset is first set to the end of the file before writing. The adjustment of the file
    /// offset and the write operation are performed as an atomic step.
    ///
    /// POSIX requires that a [`read`] that can be proved to occur after a [`write()`] has returned
    /// will return the new data. Note that not all filesystems are POSIX conforming.
    ///
    /// According to POSIX.1, if `count` is greater than [`c_ssize_t::MAX`]s, the result is
    /// implementation-defined.
    ///
    /// # Return Value
    /// On success, the number of bytes written is returned. On error, -1 is returned, and
    /// [`errno`] is set to indicate the error.
    ///
    /// Note that a successful [`write()`] may transfer fewer than count bytes. Such partial writes
    /// can occur for various reasons; for example, because there was insufficient space on the
    /// disk device to write all of the requested bytes, or because a blocked [`write()`] to a
    /// socket, pipe, or similar was interrupted by a signal handler after it had transferred
    /// some, but before it had transferred all of the requested bytes. In the event of a partial
    /// write, the caller can make another [`write()`] call to transfer the remaining bytes. The
    /// subsequent call will either transfer further bytes or may result in an error (e.g., if the
    /// disk is now full).
    ///
    /// If `count` is zero and `fd` refers to a regular file, then [`write()`] may return a failure
    /// status if one of the errors below is detected. If no errors are detected, or error
    /// detection is not performed, 0 is returned without causing any other effect. If `count` is
    /// zero and `fd` refers to a file other than a regular file, the results are not specified.
    ///
    /// # Errors
    ///  * [`EAGAIN`] - The file descriptor `fd` refers to a file other than a socket and has been
    ///                 marked nonblocking ([`O_NONBLOCK`]), and the write would block. See
    ///                 [`open`] for further details on the [`O_NONBLOCK`] flag.
    ///  * [`EAGAIN`] or [`EWOULDBLOCK`] - The file descriptor `fd` refers to a socket and has been
    ///                                    marked nonblocking ([`O_NONBLOCK`]), and the write would
    ///                                    block. POSIX.1-2001 allows either error to be returned
    ///                                    for this case, and does not require these constants to
    ///                                    have the same value, so a portable application should
    ///                                    check for both possibilities.
    ///  * [`EBADF`] - `fd` is not a valid file descriptor or is not open for writing.
    ///  * [`EDESTADDRREQ`] - `fd` refers to a datagram socket for which a peer address has not
    ///                        been set using [`connect`].
    ///  * [`EDQUOT`] - The user's quota of disk blocks on the filesystem containing the file
    ///                 referred to by `fd` has been exhausted.
    ///  * [`EFAULT`] - `buf` is outside your accessible address space.
    ///  * [`EFBIG`] - An attempt was made to write a file that exceeds the implementation-defined
    ///                maximum file size or the process's file size limit, or to write at a
    ///                position past the maximum allowed offset.
    ///  * [`EINTR`] - The call was interrupted by a signal before any data was written.
    ///  * [`EINVAL`] - `fd` is attached to an object which is unsuitable for writing; or the file
    ///                 was opened with the [`O_DIRECT`] flag, and either the address specified in
    ///                 `buf`, the value specified in `count`, or the file offset is not suitably
    ///                 aligned.
    ///  * [`EIO`] - A low-level I/O error occurred while modifying the inode. This error may
    ///              relate to the write-back of data written by an earlier [`write()`], which may
    ///              have been issued to a different file descriptor on the same file. Since Linux
    ///              4.13, errors from write-back come with a promise that they may be reported by
    ///              subsequent. [`write()`] requests, and will be reported by a subsequent [`fsync`]
    ///              (whether or not they were also reported by [`write()`]). An alternate cause of
    ///              [`EIO`] on networked filesystems is when an advisory lock had been taken out
    ///              on the file descriptor and this lock has been lost.
    ///  * [`ENOSPC`] - The device containing the file referred to by `fd` has no room for the
    ///                 data.
    ///  * [`EPERM`] - The operation was prevented by a file seal; see [`fcntl`].
    ///  * [`EPIPE`] - `fd` is connected to a pipe or socket whose reading end is closed. When this
    ///                happens the writing process will also receive a [`SIGPIPE`] signal. (Thus,
    ///                the write return value is seen only if the program catches, blocks or
    ///                ignores this signal.)
    ///
    /// Other errors may occur, depending on the object connected to `fd`.
    pub fn write(fd: c_int, buf: *const c_void, count: c_ssize_t) -> c_ssize_t;
}
