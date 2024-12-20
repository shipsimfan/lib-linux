use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::EINVAL,
    fcntl::{
        open, F_GETFL, F_SETFL, O_APPEND, O_ASYNC, O_CLOEXEC, O_CREAT, O_DIRECT, O_DSYNC, O_EXCL,
        O_NOATIME, O_NOCTTY, O_NONBLOCK, O_RDONLY, O_RDWR, O_SYNC, O_TRUNC, O_WRONLY,
    },
};
#[allow(unused_imports)]
use std::ffi::c_void;

extern "C" {
    /// Manipulate file descriptor
    ///
    /// # Description
    /// [`fcntl`] performs one of the operations described below on the open file descriptor `fd`.
    /// The operation is determined by `op`.
    ///
    /// [`fcntl`] can take an optional third argument. Whether or not this argument is required is
    /// determined by `op`. The required argument type is indicated in parentheses after each `op`
    /// name (in most cases, the required type is [`c_int`], and we identify the argument using the
    /// name `arg`), or [`c_void`] is specified if the argument is not required.
    ///
    /// Certain of the operations below are supported only since a particular Linux kernel version.
    /// The preferred method of checking whether the host kernel supports a particular operation is
    /// to invoke [`fcntl`] with the desired `op` value and then test whether the call failed with
    /// [`EINVAL`], indicating that the kernel does not recognize this value.
    ///
    /// ## Duplicating a file descriptor
    ///  * [`F_DUPFD`] - ([`c_int`]) Duplicate the file descriptor fd using the lowest-numbered
    ///                  available file descriptor greater than or equal to `arg`. This is
    ///                  different from [`dup2`], which uses exactly the file descriptor specified.
    ///                  On success, the new file descriptor is returned.
    ///  * [`F_DUPFD_CLOEXEC`] - ([`c_int`]; since Linux 2.6.24) As for [`F_DUPFD`], but
    ///                          additionally set the close-on-exec flag for the duplicate file
    ///                          descriptor. Specifying this flag permits a program to avoid an
    ///                          additional [`fcntl`] [`F_SETFD`] operation to set the
    ///                          [`FD_CLOEXEC`] flag.
    ///
    /// ## File descriptor flags
    /// The following operations manipulate the flags associated with a file descriptor. Currently,
    /// only one such flag is defined: [`FD_CLOEXEC`], the close-on-exec flag. If the
    /// [`FD_CLOEXEC`] bit is set, the file descriptor will automatically be closed during a
    /// successful [`execve`]. (If the [`execve`] fails, the file descriptor is left open.) If the
    /// [`FD_CLOEXEC`] bit is not set, the file descriptor will remain open across an [`execve`].
    ///  * [`F_GETFD`] - ([`c_void`]) Return (as the function result) the file descriptor flags;
    ///                  `arg` is ignored.
    ///  * [`F_SETFD`] - ([`c_void`]) Return (as the function result) the file descriptor flags;
    ///                  `arg` is ignored.
    ///
    /// In multithreaded programs, using [`fcntl`] [`F_SETFD`] to set the close-on-exec flag at the
    /// same time as another thread performs a [`fork`] plus [`execve`] is vulnerable to a race
    /// condition that may unintentionally leak the file descriptor to the program executed in the
    /// child process. See the discussion of the [`O_CLOEXEC`] flag in [`open`] for details and a
    /// remedy to the problem.
    ///
    /// ## File status flags
    /// Each open file description has certain associated status flags, initialized by [`open`] and
    /// possibly modified by [`fcntl`]. Duplicated file descriptors (made with [`dup`],
    /// `fcntl(F_DUPFD)`, [`fork`], etc.) refer to the same open file description, and thus share
    /// the same file status flags.
    ///
    /// The file status flags and their semantics are described in [`open`].
    ///  * [`F_GETFL`] - ([`c_void`]) Return (as the function result) the file access mode and the
    ///                  file status flags; `arg` is ignored.
    ///  * [`F_SETFL`] - ([`c_int`]) Set the file status flags to the value specified by `arg`.
    ///                  File access mode ([`O_RDONLY`], [`O_WRONLY`], [`O_RDWR`]) and file
    ///                  creation flags (i.e., [`O_CREAT`], [`O_EXCL`], [`O_NOCTTY`], [`O_TRUNC`])
    ///                  in `arg` are ignored.  On Linux, this operation can change only the
    ///                  [`O_APPEND`], [`O_ASYNC`], [`O_DIRECT`], [`O_NOATIME`], and [`O_NONBLOCK`]
    ///                  flags. It is not possible to change the [`O_DSYNC`] and [`O_SYNC`] flags.
    pub fn fcntl(fd: c_int, op: c_int, ...) -> c_int;
}
