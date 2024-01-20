use crate::sys::stat::mode_t;
use std::ffi::{c_char, c_int};

// rustdoc imports
#[allow(unused_imports)]
use crate::errno::{
    errno, EACCES, EFAULT, EIO, ELOOP, ENAMETOOLONG, ENOENT, ENOMEM, ENOTDIR, EPERM, EROFS,
};

extern "C" {
    /// [`chmod`] changes the permissions of the file specified whose pathname is given in
    /// `pathname`, which is dereferenced if it is a symbolic link.
    ///
    /// The new file mode is specified in `mode`, which is a bit mask created by ORing together
    /// zero or more of the following:
    ///  * [`S_ISUID`] (0o4000) - set-user-ID (set process effective user ID on [`execve`])
    ///  * [`S_ISGID`] (0o2000) - set-group-ID (set process effective group ID on [`execve`];
    ///                           mandatory locking, as described in fcntl(2); take a new file's
    ///                           group from parent directory, as described in [`chown`] and
    ///                           [`mkdir`])
    ///  * [`S_ISVTX`] (0o1000) - sticky bit (restricted deletion flag, as described in [`unlink`])
    ///  * [`S_IRUSR`] (0o0400) - read by owner
    ///  * [`S_IWUSR`] (0o0200) - write by owner
    ///  * [`S_IXUSR`] (0o0100) - execute/search by owner ("search" applies for directories, and
    ///                           means that entries within the directory can be accessed)
    ///  * [`S_IRGRP`] (0o0040) - read by group
    ///  * [`S_IWGRP`] (0o0020) - write by group
    ///  * [`S_IXGRP`] (0o0010) - execute/search by group
    ///  * [`S_IROTH`] (0o0004) - read by others
    ///  * [`S_IWOTH`] (0o0002) - write by others
    ///  * [`S_IXOTH`] (0o0001) - execute/search by others
    ///
    /// The effective UID of the calling process must match the owner of the file, or the process
    /// must be privileged (Linux: it must have the [`CAP_FOWNER`] capability).
    ///
    /// If the calling process is not privileged (Linux: does not have the [`CAP_FSETID`]
    /// capability), and the group of the file does not match the effective group ID of the process
    /// or one of its supplementary group IDs, the [`S_ISGID`] bit will be turned off, but this
    /// will not cause an error to be returned.
    ///
    /// As a security measure, depending on the filesystem, the `set-user-ID` and `set-group-ID`
    /// execution bits may be turned off if a file is written. (On Linux, this occurs if the
    /// writing process does not have the [`CAP_FSETID`] capability.) On some filesystems, only the
    /// superuser can set the sticky bit, which may have a special meaning. For the sticky bit, and
    /// for `set-user-ID` and `set-group-ID` bits on directories.
    ///
    /// On NFS filesystems, restricting the permissions will immediately influence already open
    /// files, because the access control is done on the server, but open files are maintained by
    /// the client. Widening the permissions may be delayed for other clients if attribute caching
    /// is enabled on them.
    ///
    /// # Return Value
    /// On success, zero is returned. On error, -1 is returned, and [`errno`] is set appropriately.
    ///
    /// # Errors
    /// Depending on the filesystem, errors other than those listed below can be returned.
    ///
    /// The more general errors for [`chmod`] are listed below:
    ///  * [`EACCES`] - Search permission is denied on a component of the path prefix.
    ///  * [`EFAULT`] - `pathname` points outside your accessible address space.
    ///  * [`EIO`] - An I/O error occurred.
    ///  * [`ELOOP`] - Too many symbolic links were encountered in resolving `pathname`.
    ///  * [`ENAMETOOLONG`] - `pathname` is too long.
    ///  * [`ENOENT`] - The file does not exist.
    ///  * [`ENOMEM`] - Insufficient kernel memory was available.
    ///  * [`ENOTDIR`] - A component of the path prefix is not a directory.
    ///  * [`EPERM`] - The effective UID does not match the owner of the file, and the process is
    ///                not privileged (Linux: it does not have the [`CAP_FOWNER`] capability).
    ///  * [`EPERM`] - The file is marked immutable or append-only.
    ///  * [`EROFS`] - The named file resides on a read-only filesystem.
    pub fn chmod(pathname: *const c_char, mode: mode_t) -> c_int;
}
