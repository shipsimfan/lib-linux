use std::ffi::c_char;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{
        EACCES, EBUSY, EFAULT, EIO, EISDIR, ELOOP, ENAMETOOLONG, ENOENT, ENOMEM, ENOTDIR, EPERM,
        EROFS, errno,
    },
    linux::capability::CAP_FOWNER,
    sys::stat::S_ISVTX,
};

unsafe extern "C" {
    /// Delete a name and possibly the file it refers to
    ///
    /// # Description
    /// [`unlink`] deletes a name from the filesystem. If that name was the last link to a file and
    /// no processes have the file open, the file is deleted and the space it was using is made
    /// available for reuse.
    ///
    /// If the name was the last link to a file but any processes still have the file open, the
    /// file will remain in existence until the last file descriptor referring to it is closed.
    ///
    /// If the name referred to a symbolic link, the link is removed.
    ///
    /// If the name referred to a socket, FIFO, or device, the name for it is removed but processes
    /// which have the object open may continue to use it.
    ///
    /// # Return Value
    /// On success, zero is returned. On error, -1 is returned, and [`errno`] is set to indicate
    /// the error.
    ///
    /// # Errors
    ///  * [`EACCES`] - Write access to the directory containing `pathname` is not allowed for the
    ///                 process's effective UID, or one of the directories in `pathname` did not
    ///                 allow search permission.
    ///  * [`EBUSY`] - The file `pathname` cannot be unlinked because it is being used by the
    ///                system or another process; for example, it is a mount point or the NFS
    ///                client software created it to represent an active but otherwise nameless
    ///                inode ("NFS silly renamed").
    ///  * [`EFAULT`] - `pathname` points outside your accessible address space.
    ///  * [`EIO`] - An I/O error occurred.
    ///  * [`EISDIR`] - `pathname` refers to a directory. (This is the non-POSIX value returned by
    ///                 Linux since 2.1.132.)
    ///  * [`ELOOP`] - Too many symbolic links were encountered in translating `pathname`.
    ///  * [`ENAMETOOLONG`] - `pathname` was too long.
    ///  * [`ENOENT`] - A component in `pathname` does not exist or is a dangling symbolic link, or
    ///                 `pathname` is empty.
    ///  * [`ENOMEM`] - Insufficient kernel memory was available.
    ///  * [`ENOTDIR`] - A component used as a directory in `pathname` is not, in fact, a
    ///                  directory.
    ///  * [`EPERM`] - The system does not allow unlinking of directories, or unlinking of
    ///                directories requires privileges that the calling process doesn't have. (This
    ///                is the POSIX prescribed error return; as noted above, Linux returns
    ///                [`EISDIR`] for this case.)
    ///  * [`EPERM`] (Linux only) - The file system does not allow unlinking of files.
    ///  * [`EPERM`] or [`EACCES`] - The directory containing `pathname` has the sticky bit
    ///                              ([`S_ISVTX`]) set and the process's effective UID is neither
    ///                              the UID of the file to be deleted nor that of the directory
    ///                              containing it, and the process is not privileged (Linux: does
    ///                              not have the [`CAP_FOWNER`] capability).
    ///  * [`EROFS`] - `pathname` refers to a file on a read-only file system.
    pub fn unlink(pathname: *const c_char);
}
