use crate::sys::stat::Statx;
use std::ffi::{c_char, c_int, c_uint};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, EACCES, EBADF, EFAULT, EINVAL, ELOOP, ENAMETOOLONG, ENOENT, ENOMEM, ENOTDIR},
    fcntl::{
        AT_EMPTY_PATH, AT_FDCWD, AT_NO_AUTOMOUNT, AT_STATX_DONT_SYNC, AT_STATX_FORCE_SYNC,
        AT_STATX_SYNC_AS_STAT, AT_SYMLINK_NOFOLLOW,
    },
    sys::stat::{
        STATX_ATIME, STATX_BASIC_STATS, STATX_BLOCKS, STATX_BTIME, STATX_CTIME, STATX_DIOALIGN,
        STATX_GID, STATX_INO, STATX_MNT_ID, STATX_MODE, STATX_MTIME, STATX_NLINK, STATX_SIZE,
        STATX_TYPE, STATX_UID, STATX__RESERVED,
    },
};
#[allow(unused_imports)]
use std::ptr::null;

extern "C" {
    /// Get file status
    ///
    /// # Description
    /// This function returns information about a file, storing it in the buffer pointed to by
    /// `statxbuf`. The returned buffer is a structure of the [`Statx`] type.
    ///
    /// To access a file's status, no permissions are required on the file itself, but in the case
    /// of [`statx`] with a pathname, execute (search) permission is required on all of the
    /// directories in `pathname` that lead to the file.
    ///
    /// [`statx`] uses `pathname`, `dirfd`, and `flags` to identify the target file in one of the
    /// following ways:
    ///  * An absolute pathname - If `pathname` begins with a slash, then it is an absolute
    ///                           pathname that identifies the target file.  In this case, `dirfd`
    ///                           is ignored.
    ///  * A relative pathname - If `pathname` is a string that begins with a character other than
    ///                          a slash and `dirfd` is [`AT_FDCWD`], then `pathname` is a relative
    ///                          pathname that is interpreted relative to the process's current
    ///                          working directory.
    ///  * A directory-relative pathname - If `pathname` is a string that begins with a character
    ///                                    other than a slash and `dirfd` is a file descriptor that
    ///                                    refers to a directory, then `pathname` is a relative
    ///                                    pathname that is interpreted relative to the directory
    ///                                    referred to by `dirfd`.
    ///  * By file descriptor - If `pathname` is an empty string and the [`AT_EMPTY_PATH`] flag is
    ///                         specified in flags (see below), then the target file is the one
    ///                         referred to by the file descriptor `dirfd`.
    ///
    /// `flags` can be used to influence a pathname-based lookup.  A value for `flags` is
    /// constructed by ORing together zero or more of the following constants:
    ///  * [`AT_EMPTY_PATH`] - If `pathname` is an empty string, operate on the file referred to by
    ///                        `dirfd` (which may have been obtained using the [`open`] [`O_PATH`]
    ///                        flag). In this case, `dirfd` can refer to any type of file, not just
    ///                        a directory. If `dirfd` is [`AT_FDCWD`], the call operates on the
    ///                        current working directory.
    ///  * [`AT_NO_AUTOMOUNT`] - Don't automount the terminal ("basename") component of `pathname`
    ///                          if it is a directory that is an automount point. This allows the
    ///                          caller to gather attributes of an automount point (rather than the
    ///                          location it would mount). This flag has no effect if the mount
    ///                          point has already been mounted over. The [`AT_NO_AUTOMOUNT`] flag
    ///                          can be used in tools that scan directories to prevent
    ///                          mass-automounting of a directory of automount points. All of
    ///                          [`stat`], [`lstat`], and [`fstatat`] act as though
    ///                          [`AT_NO_AUTOMOUNT`] was set.
    ///  * [`AT_SYMLINK_NOFOLLOW`] - If `pathname` is a symbolic link, do not dereference it:
    ///                              instead return information about the link itself, like
    ///                              [`lstat`].
    ///
    /// `flags` can also be used to control what sort of synchronization the kernel will do when
    /// querying a file on a remote filesystem. This is done by ORing in one of the following
    /// values:
    ///  * [`AT_STATX_SYNC_AS_STAT`] - Do whatever [`stat`] does. This is the default and is very
    ///                                much filesystem-specific.
    ///  * [`AT_STATX_FORCE_SYNC`] - Force the attributes to be synchronized with the server. This
    ///                              may require that a network filesystem perform a data writeback
    ///                              to get the timestamps correct.
    ///  * [`AT_STATX_DONT_SYNC`] - Don't synchronize anything, but rather just take whatever the
    ///                             system has cached if possible. This may mean that the
    ///                             information returned is approximate, but, on a network
    ///                             filesystem, it may not involve a round trip to the server -
    ///                             even if no lease is held.
    ///
    /// The `mask` argument to [`statx`] is used to tell the kernel which fields the caller is
    /// interested in. `mask` is an ORed combination of the following constants:
    ///  * [`STATX_TYPE`] - Want `mode & S_IFMT`
    ///  * [`STATX_MODE`] - Want `mode & ~S_IFMT`
    ///  * [`STATX_NLINK`] - Want `nlink`
    ///  * [`STATX_UID`] - Want `uid`
    ///  * [`STATX_GID`] - Want `gid`
    ///  * [`STATX_ATIME`] - Want `atime`
    ///  * [`STATX_MTIME`] - Want `mtime`
    ///  * [`STATX_CTIME`] - Want `ctime`
    ///  * [`STATX_INO`] - Want `ino`
    ///  * [`STATX_SIZE`] - Want `size`
    ///  * [`STATX_BLOCKS`] - Want `blocks`
    ///  * [`STATX_BASIC_STATS`] - All of the above
    ///  * [`STATX_BTIME`] - Want `btime`
    ///  * [`STATX_MNT_ID`] - Want `mnt_id` (since Linux 5.8)
    ///  * [`STATX_DIOALIGN`] - Want `dio_mem_align` and `dio_offset_align` (since Linux 6.1;
    ///                         support varies by filesystem)
    ///
    /// Note that, in general, the kernel does not reject values in `mask` other than the above.
    /// (For an exception, see [`EINVAL`] in errors.) Instead, it simply informs the caller which
    /// values are supported by this kernel and filesystem via the `Statx.mask` field. Therefore,
    /// do not simply set mask to [`c_int::MAX`] (all bits set), as one or more bits may, in the
    /// future, be used to specify an extension to the buffer.
    ///
    /// # Return Value
    /// On success, zero is returned. On error, -1 is returned, and [`errno`] is set to indicate
    /// the error.
    ///
    /// # Errors
    ///  * [`EACCES`] - Search permission is denied for one of the directories in the path prefix
    ///                 of `pathname`.
    ///  * [`EBADF`] - `pathname` is relative but `dirfd` is neither [`AT_FDCWD`] nor a valid file
    ///                descriptor.
    ///  * [`EFAULT`] - `pathname` or `statxbuf` is [`null`] or points to a location outside the
    ///                 process's accessible address space.
    ///  * [`EINVAL`] - Invalid flag specified in `flags`.
    ///  * [`EINVAL`] - Reserved flag specified in `mask`. (Currently, there is one such flag,
    ///                 designated by the constant [`STATX__RESERVED`], with the value
    ///                 `0x80000000`.)
    ///  * [`ELOOP`] - Too many symbolic links encountered while traversing the `pathname`.
    ///  * [`ENAMETOOLONG`] - `pathname` is too long.
    ///  * [`ENOENT`] - A component of `pathname` does not exist, or `pathname` is an empty string |
    ///                 and [`AT_EMPTY_PATH`] was not specified in `flags`.
    ///  * [`ENOMEM`] - Out of memory (i.e., kernel memory).
    ///  * [`ENOTDIR`] - A component of the path prefix of `pathname` is not a directory or
    ///                 `pathname` is relative and `dirfd` is a file descriptor referring to a file
    ///                 other than a directory.
    pub fn statx(
        dirfd: c_int,
        pathname: *const c_char,
        flags: c_int,
        mask: c_uint,
        statxbuf: *mut Statx,
    ) -> c_int;
}
