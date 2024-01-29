use std::ffi::{c_char, c_int};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{
        errno, EACCES, EBADF, EBUSY, EDQUOT, EEXIST, EFAULT, EFBIG, EINTR, EINVAL, EISDIR, ELOOP,
        EMFILE, ENAMETOOLONG, ENFILE, ENODEV, ENOENT, ENOMEM, ENOSPC, ENOTDIR, ENXIO, EOPNOTSUPP,
        EOVERFLOW, EPERM, EROFS, ETXTBSY, EWOULDBLOCK,
    },
    fcntl::{
        O_APPEND, O_ASYNC, O_CLOEXEC, O_CREAT, O_DIRECT, O_DIRECTORY, O_DSYNC, O_EXCL, O_LARGEFILE,
        O_NDELAY, O_NOATIME, O_NOCTTY, O_NOFOLLOW, O_NONBLOCK, O_PATH, O_RDONLY, O_RDWR, O_SYNC,
        O_TMPFILE, O_TRUNC, O_WRONLY,
    },
    sys::epoll,
    unistd::{close, fdatasync, fsync, read, write},
};

extern "C" {
    /// The [`open`] system call opens the file specified by `pathname`. If the specified file does
    /// not exist, it may optionally (if [`O_CREAT`] is specified in `flags`) be created by
    /// [`open`].
    ///
    /// The return value of [`open`] is a file descriptor, a small, nonnegative integer that is an
    /// index to an entry in the process's table of open file descriptors. The file descriptor is
    /// used in subsequent system calls ([`read`], [`write()`], [`lseek`], [`fcntl`], etc.) to refer
    /// to the open file. The file descriptor returned by a successful call will be the
    /// lowest-numbered file descriptor not currently open for the process.
    ///
    /// By default, the new file descriptor is set to remain open across an [`execve`] (i.e., the
    /// [`FD_CLOEXEC`] file descriptor flag described in [`fcntl`] is initially disabled); the
    /// [`O_CLOEXEC`] flag, described below, can be used to change this default. The file offset
    /// is set to the beginning of the file (see [`lseek`]).
    ///
    /// A call to [`open`] creates a new open file description, an entry in the system-wide table
    /// of open files. The open file description records the file offset and the file status flags
    /// (see below). A file descriptor is a reference to an open file description; this reference
    /// is unaffected if `pathname` is subsequently removed or modified to refer to a different
    /// file.
    ///
    /// The argument `flags` must include one of the following access modes: [`O_RDONLY`],
    /// [`O_WRONLY`], or [`O_RDWR`]. These request opening the file read-only, write-only, or
    /// read/write, respectively.
    ///
    /// In addition, zero or more file creation flags and file status flags can be bitwise ORed in
    /// `flags`. The file creation flags are [`O_CLOEXEC`], [`O_CREAT`], [`O_DIRECTORY`],
    /// [`O_EXCL`], [`O_NOCTTY`], [`O_NOFOLLOW`], [`O_TMPFILE`], and [`O_TRUNC`]. The file status
    /// flags are all of the remaining flags listed below. The distinction between these two groups
    /// of flags is that the file creation flags affect the semantics of the open operation itself,
    /// while the file status flags affect the semantics of subsequent I/O operations. The file
    /// status flags can be retrieved and (in some cases) modified; see [`fcntl`] for details.
    ///
    /// The full list of file creation flags and file status flags is as follows:
    ///  * [`O_APPEND`] - The file is opened in append mode. Before each [`write()`], the file offset
    ///                   is positioned at the end of the file, as if with [`lseek`]. The
    ///                   modification of the file offset and the write operation are performed as
    ///                   a single atomic step. [`O_APPEND`] may lead to corrupted files on NFS
    ///                   filesystems if more than one process appends data to a file at once. This
    ///                   is because NFS does not support appending to a file, so the client kernel
    ///                   has to simulate it, which can't be done without a race condition.
    ///  * [`O_ASYNC`] - Enable signal-driven I/O: generate a signal ([`SIGIO`] by default, but
    ///                  this can be changed via [`fcntl`]) when input or output becomes possible
    ///                  on this file descriptor. This feature is available only for terminals,
    ///                  pseudoterminals, sockets, and (since Linux 2.6) pipes and FIFOs. See
    ///                  [`fcntl`] for further details.
    ///  * [`O_CLOEXEC`] (since Linux 2.6.23) - Enable the close-on-exec flag for the new file
    ///                                         descriptor. Specifying this flag permits a program
    ///                                         to avoid additional [`fcntl`] [`F_SETFD`]
    ///                                         operations to set the [`FD_CLOEXEC`] flag. Note
    ///                                         that the use of this flag is essential in some
    ///                                         multithreaded programs, because using a separate
    ///                                         [`fcntl`] [`F_SETFD`] operation to set the
    ///                                         [`FD_CLOEXEC`] flag does not suffice to avoid race
    ///                                         conditions where one thread opens a file descriptor
    ///                                         and attempts to set its close-on-exec flag using
    ///                                         [`fcntl`] at the same time as another thread does a
    ///                                         [`fork`] plus [`execve`]. Depending on the order of
    ///                                         execution, the race may lead to the file descriptor
    ///                                         returned by [`open`] being unintentionally leaked
    ///                                         to the program executed by the child process
    ///                                         created by [`fork`]. (This kind of race is in
    ///                                         principle possible for any system call that creates
    ///                                         a file descriptor whose close-on-exec flag should
    ///                                         be set, and various other Linux system calls
    ///                                         provide an equivalent of the [`O_CLOEXEC`] flag to
    ///                                         deal with this problem.)
    ///  * [`O_CREAT`] - If `pathname` does not exist, create it as a regular file. The owner (user
    ///                  ID) of the new file is set to the effective user ID of the process. The
    ///                  group ownership (group ID) of the new file is set either to the effective
    ///                  group ID of the process (System V semantics) or to the group ID of the
    ///                  parent directory (BSD semantics). On Linux, the behavior depends on
    ///                  whether the set-group-ID mode bit is set on the parent directory: if that
    ///                  bit is set, then BSD semantics apply; otherwise, System V semantics apply.
    ///                  For some filesystems, the behavior also depends on the bsdgroups and
    ///                  sysvgroups mount options described in [`mount`]. The `mode` argument
    ///                  specifies the file mode bits to be applied when a new file is created. If
    ///                  neither [`O_CREAT`] nor [`O_TMPFILE`] is specified in flags, then `mode`
    ///                  is ignored (and can thus be specified as 0, or simply omitted). The `mode`
    ///                  argument must be supplied if [`O_CREAT`] or [`O_TMPFILE`] is specified in
    ///                  flags; if it is not supplied, some arbitrary bytes from the stack will be
    ///                  applied as the file mode. The effective mode is modified by the process's
    ///                  umask in the usual way: in the absence of a default ACL, the mode of the
    ///                  created file is `(mode & ~umask)`. Note that mode applies only to future
    ///                  accesses of the newly created file; the open() call that creates a read-
    ///                  only file may well return a read/write file descriptor. The following
    ///                  symbolic constants are provided for mode:
    ///    * [`S_IRWXU`] - `00700` - user (file owner) has read, write, and execute permission
    ///    * [`S_IRUSR`] - `00400` - user has read permission
    ///    * [`S_IWUSR`] - `00200` - user has write permission
    ///    * [`S_IXUSR`] - `00100` - user has execute permission
    ///    * [`S_IRWXG`] - `00070` - group has read, write, and execute permission
    ///    * [`S_IRGRP`] - `00040` - group has read permission
    ///    * [`S_IWGRP`] - `00020` - group has write permission
    ///    * [`S_IXGRP`] - `00010` - group has execute permission
    ///    * [`S_IRWXO`] - `00007` - others have read, write, and execute permission
    ///    * [`S_IROTH`] - `00004` - others have read permission
    ///    * [`S_IWOTH`] - `00002` - others have write permission
    ///    * [`S_IXOTH`] - `00001` - others have execute permission
    ///    * [`S_ISUID`] - `0004000` - set-user-ID bit
    ///    * [`S_ISGID`] - `0002000` - set-group-ID bit
    ///    * [`S_ISVTX`] - `0001000` - sticky bit
    ///  * [`O_DIRECT`] (since Linux 2.4.10) - Try to minimize cache effects of the I/O to and from
    ///                                        this file. In general this will degrade performance,
    ///                                        but it is useful in special situations, such as when
    ///                                        applications do their own caching. File I/O is done
    ///                                        directly to/from user-space buffers. The
    ///                                        [`O_DIRECT`] flag on its own makes an effort to
    ///                                        transfer data synchronously, but does not give the
    ///                                        guarantees of the [`O_SYNC`] flag that data and
    ///                                        necessary metadata are transferred. To guarantee
    ///                                        synchronous I/O, [`O_SYNC`] must be used in addition
    ///                                        to [`O_DIRECT`].
    ///  * [`O_DIRECTORY`] - If `pathname` is not a directory, cause the open to fail. This flag was
    ///                     added in Linux 2.1.126, to avoid denial-of-service problems if
    ///                     [`opendir`] is called on a FIFO or tape device.
    ///  * [`O_DSYNC`] - Write operations on the file will complete according to the requirements
    ///                  of synchronized I/O data integrity completion. By the time [`write()`] (and
    ///                  similar) return, the output data has been transferred to the underlying
    ///                  hardware, along with any file metadata that would be required to retrieve
    ///                  that data (i.e., as though each [`write()`] was followed by a call to
    ///                  [`fdatasync`]).
    ///  * [`O_EXCL`] - Ensure that this call creates the file: if this flag is specified in
    ///                 conjunction with [`O_CREAT`], and `pathname` already exists, then [`open`]
    ///                 fails with the error [`EEXIST`]. When these two flags are specified,
    ///                 symbolic links are not followed: if `pathname` is a symbolic link, then
    ///                 [`open`] fails regardless of where the symbolic link points. In general,
    ///                 the behavior of [`O_EXCL`] is undefined if it is used without [`O_CREAT`].
    ///                 There is one exception: on Linux 2.6 and later, [`O_EXCL`] can be used
    ///                 without [`O_CREAT`] if `pathname` refers to a block device. If the block
    ///                 device is in use by the system (e.g., mounted), [`open`] fails with the
    ///                 error [`EBUSY`]. On NFS, [`O_EXCL`] is supported only when using NFSv3 or
    ///                 later on kernel 2.6 or later. In NFS environments where [`O_EXCL`] support
    ///                 is not provided, programs that rely on it for performing locking tasks will
    ///                 contain a race condition. Portable programs that want to perform atomic
    ///                 file locking using a lockfile, and need to avoid reliance on NFS support
    ///                 for [`O_EXCL`], can create a unique file on the same filesystem (e.g.,
    ///                 incorporating hostname and PID), and use [`link`] to make a link to the
    ///                 lockfile. If [`link`] returns 0, the lock is successful. Otherwise, use
    ///                 [`stat`] on the unique file to check if its link count has increased to 2,
    ///                 in which case the lock is also successful.
    ///  * [`O_LARGEFILE`] - (LFS) Allow files whose sizes cannot be represented in an [`off_t`]
    ///                      (but can be represented in an [`off64_t`]) to be opened. The
    ///                      [`_LARGEFILE64_SOURCE`] macro must be defined (before including any
    ///                      header files) in order to obtain this definition. Setting the
    ///                      [`_FILE_OFFSET_BITS`] feature test macro to 64 (rather than using
    ///                      [`O_LARGEFILE`]) is the preferred method of accessing large files on
    ///                      32-bit systems.
    ///  * [`O_NOATIME`] (since Linux 2.6.8) - Do not update the file last access time (`st_atime`
    ///                                        in the inode) when the file is [`read`]. This flag
    ///                                        is intended for use by indexing or backup programs,
    ///                                        where its use can significantly reduce the amount
    ///                                        of disk activity. This flag may not be effective on
    ///                                        all filesystems. One example is NFS, where the
    ///                                        server maintains the access time. This flag can be
    ///                                        employed only if one of the following conditions is
    ///                                        true:
    ///    * The effective UID of the process matches the owner UID of the file.
    ///    * The calling process has the CAP_FOWNER capability in its user namespace and the owner
    ///      UID of the file has a mapping in the namespace.
    ///  * [`O_NOCTTY`] - If `pathname` refers to a terminal device, it will not become the
    ///                  process's controlling terminal even if the process does not have one.
    ///  * [`O_NOFOLLOW`] - If the trailing component (i.e., basename) of `pathname` is a symbolic
    ///                     link, then the open fails, with the error [`ELOOP`]. Symbolic links in
    ///                     earlier components of the `pathname` will still be followed. (Note that
    ///                     the [`ELOOP`] error that can occur in this case is indistinguishable
    ///                     from the case where an open fails because there are too many symbolic
    ///                     links found while resolving components in the prefix part of the
    ///                     `pathname`.) This flag is a FreeBSD extension, which was added in Linux
    ///                     2.1.126, and has subsequently been standardized in POSIX.1-2008. See
    ///                     also [`O_PATH`] below.   
    ///  * [`O_NONBLOCK`] or [`O_NDELAY`] - When possible, the file is opened in nonblocking mode.
    ///                                     Neither the [`open`] nor any subsequent I/O operations
    ///                                     on the file descriptor which is returned will cause the
    ///                                     calling process to wait. Note that the setting of this
    ///                                     flag has no effect on the operation of [`poll`],
    ///                                     [`select`], [`epoll`], and similar, since those
    ///                                     interfaces merely inform the caller about whether a
    ///                                     file descriptor is "ready", meaning that an I/O
    ///                                     operation performed on the file descriptor with the
    ///                                     [`O_NONBLOCK`] flag clear would not block. Note that
    ///                                     this flag has no effect for regular files and block
    ///                                     devices; that is, I/O operations will (briefly) block
    ///                                     when device activity is required, regardless of whether
    ///                                     [`O_NONBLOCK`] is set. Since [`O_NONBLOCK`] semantics
    ///                                     might eventually be implemented, applications should
    ///                                     not depend upon blocking behavior when specifying this
    ///                                     flag for regular files and block devices. For a
    ///                                     discussion of the effect of [`O_NONBLOCK`] in
    ///                                     conjunction with mandatory file locks and with file
    ///                                     leases, see [`fcntl`].
    ///  * [`O_PATH`] (since Linux 2.6.39) - Obtain a file descriptor that can be used for two
    ///                                      purposes: to indicate a location in the filesystem
    ///                                      tree and to perform operations that act purely at the
    ///                                      file descriptor level. The file itself is not opened,
    ///                                      and other file operations (e.g., [`read`], [`write()`],
    ///                                      [`fchmod`], [`fchown`], [`fgetxattr`], [`ioctl`],
    ///                                      [`mmap`]) fail with the error [`EBADF`]. When
    ///                                      [`O_PATH`] is specified in flags, flag bits other than
    ///                                      [`O_CLOEXEC`], [`O_DIRECTORY`], and [`O_NOFOLLOW`] are
    ///                                      ignored. Opening a file or directory with the
    ///                                      [`O_PATH`] flag requires no permissions on the object
    ///                                      itself (but does require execute permission on the
    ///                                      directories in the path prefix). Depending on the
    ///                                      subsequent operation, a check for suitable file
    ///                                      permissions may be performed (e.g., [`fchdir`]
    ///                                      requires execute permission on the directory referred
    ///                                      to by its file descriptor argument). By contrast,
    ///                                      obtaining a reference to a filesystem object by
    ///                                      opening it with the [`O_RDONLY`] flag requires that
    ///                                      the caller have read permission on the object, even
    ///                                      when the subsequent operation (e.g., [`fchdir`],
    ///                                      [`fstat`]) does not require read permission on the
    ///                                      object. If `pathname` is a symbolic link and the
    ///                                      [`O_NOFOLLOW`] flag is also specified, then the call
    ///                                      returns a file descriptor referring to the symbolic
    ///                                      link. This file descriptor can be used as the `dirfd`
    ///                                      argument in calls to [`fchownat`], [`fstatat`],
    ///                                      [`linkat`], and [`readlinkat`] with an empty
    ///                                      `pathname` to have the calls operate on the symbolic
    ///                                      link. If `pathname` refers to an automount point that
    ///                                      has not yet been triggered, so no other filesystem is
    ///                                      mounted on it, then the call returns a file
    ///                                      descriptor referring to the automount directory
    ///                                      without triggering a mount. [`fstatfs`] can then be
    ///                                      used to determine if it is, in fact, an untriggered
    ///                                      automount point `(.f_type == AUTOFS_SUPER_MAGIC)`. One
    ///                                      use of [`O_PATH`] for regular files is to provide the
    ///                                      equivalent of POSIX.1's [`O_EXEC`] functionality. This
    ///                                      permits us to open a file for which we have execute
    ///                                      permission but not read permission, and then execute
    ///                                      that file. An [`O_PATH`] file descriptor can also be
    ///                                      passed as the argument of [`fexecve`]. The following
    ///                                      operations can be performed on the resulting file
    ///                                      descriptor:
    ///    * [`close`].
    ///    * [`fchdir`], if the file descriptor refers to a directory (since Linux 3.5).
    ///    * [`fstat`] (since Linux 3.6).
    ///    * [`fstatfs`] (since Linux 3.12).
    ///    * Duplicating the file descriptor ([`dup`], [`fcntl`] [`F_DUPFD`], etc.).
    ///    * Getting and setting file descriptor flags ([`fcntl`] [`F_GETFD`] and [`F_SETFD`]).
    ///    * Retrieving open file status flags using the [`fcntl`] [`F_GETFL`] operation: the
    ///      returned flags will include the bit [`O_PATH`].
    ///    * Passing the file descriptor as the `dirfd` argument of [`openat`] and the other
    ///      "*at()" system calls. This includes [`linkat`] with [`AT_EMPTY_PATH`] (or via procfs
    ///      using [`AT_SYMLINK_FOLLOW`]) even if the file is not a directory.
    ///    * Passing the file descriptor to another process via a UNIX domain socket.
    ///  * [`O_SYNC`] - Write operations on the file will complete according to the requirements of
    ///                 synchronized I/O file integrity completion (by contrast with the
    ///                 synchronized I/O data integrity completion provided by [`O_DSYNC`].) By the
    ///                 time [`write()`] (or similar) returns, the output data and associated file
    ///                 metadata have been transferred to the underlying hardware (i.e., as though
    ///                 each [`write()`] was followed by a call to [`fsync`]).
    ///  * [`O_TMPFILE`] (since Linux 3.11) - Create an unnamed temporary regular file. The
    ///                                       `pathname` argument specifies a directory; an unnamed
    ///                                       inode will be created in that directory's filesystem.
    ///                                       Anything written to the resulting file will be lost
    ///                                       when the last file descriptor is closed, unless the
    ///                                       file is given a name. [`O_TMPFILE`] must be specified
    ///                                       with one of [`O_RDWR`] or [`O_WRONLY`] and,
    ///                                       optionally, [`O_EXCL`]. If [`O_EXCL`] is not
    ///                                       specified, then [`linkat`] can be used to link the
    ///                                       temporary file into the filesystem, making it
    ///                                       permanent. In this case, the [`open`] `mode` argument
    ///                                       determines the file permission mode, as with
    ///                                       [`O_CREAT`]. Specifying [`O_EXCL`] in conjunction
    ///                                       with [`O_TMPFILE`] prevents a temporary file from
    ///                                       being linked into the filesystem in the above manner.
    ///                                       (Note that the meaning of [`O_EXCL`] in this case is
    ///                                       different from the meaning of [`O_EXCL`] otherwise.)
    ///                                       [`O_TMPFILE`] requires support by the underlying
    ///                                       filesystem; only a subset of Linux filesystems
    ///                                       provide that support. In the initial implementation,
    ///                                       support was provided in the ext2, ext3, ext4, UDF,
    ///                                       Minix, and tmpfs filesystems. Support for other
    ///                                       filesystems has subsequently been added as follows:
    ///                                       XFS (Linux 3.15); Btrfs (Linux 3.16); F2FS (Linux
    ///                                       3.16); and ubifs (Linux 4.9) There are two main use
    ///                                       cases for [`O_TMPFILE`]:
    ///    *  Improved [`tmpfile`] functionality: race-free creation of temporary files that (1)
    ///       are automatically deleted when closed; (2) can never be reached via any pathname; (3)
    ///       are not subject to symlink attacks; and (4) do not require the caller to devise
    ///       unique names.
    ///    *  Creating a file that is initially invisible, which is then populated with data and
    ///       adjusted to have appropriate filesystem attributes ([`fchown`], [`fchmod`],
    ///       [`fsetxattr`], etc.) before being atomically linked into the filesystem in a fully
    ///       formed state (using [`linkat`] as described above).
    ///  * [`O_TRUNC`] - If the file already exists and is a regular file and the access mode
    ///                  allows writing (i.e., is [`O_RDWR`] or [`O_WRONLY`]) it will be truncated
    ///                  to length 0. If the file is a FIFO or terminal device file, the
    ///                  [`O_TRUNC`] flag is ignored. Otherwise, the effect of [`O_TRUNC`] is
    ///                  unspecified.
    ///
    /// # Return Value
    /// On success [`open`] returns the new file descriptor (a nonnegative integer). On error, -1
    /// is returned and [`errno`] is set to indicate the error.
    ///
    /// # Errors
    /// [`open`] can fail with the following errors:
    ///  * [`EACCES`] - The requested access to the file is not allowed, or search permission is
    ///                 denied for one of the directories in the path prefix of `pathname`, or the
    ///                 file did not exist yet and write access to the parent directory is not
    ///                 allowed.
    ///  * [`EACCES`] - Where [`O_CREAT`] is specified, the `protected_fifos` or
    ///                 `protected_regular` sysctl is enabled, the file already exists and is a
    ///                 FIFO or regular file, the owner of the file is neither the current user nor
    ///                 the owner of the containing directory, and the containing directory is both
    ///                 world- or group-writable and sticky.
    ///  * [`EBUSY`] - [`O_EXCL`] was specified in `flags` and `pathname` refers to a block device
    ///                that is in use by the system (e.g., it is mounted).
    ///  * [`EDQUOT`] - Where [`O_CREAT`] is specified, the file does not exist, and the user's
    ///                 quota of disk blocks or inodes on the filesystem has been exhausted.
    ///  * [`EEXIST`] - `pathname` already exists and [`O_CREAT`] and [`O_EXCL`] were used.
    ///  * [`EFAULT`] - `pathname` points outside your accessible address space.
    ///  * [`EFBIG`] - See [`EOVERFLOW`].
    ///  * [`EINTR`] - While blocked waiting to complete an open of a slow device (e.g., a FIFO)
    ///                the call was interrupted by a signal handler.
    ///  * [`EINVAL`] - The filesystem does not support the [`O_DIRECT`] flag.
    ///  * [`EINVAL`] - Invalid value in `flags`.
    ///  * [`EINVAL`] - [`O_TMPFILE`] was specified in `flags`, but neither [`O_WRONLY`] nor
    ///                 [`O_RDWR`] was specified.
    ///  * [`EINVAL`] - [`O_CREAT`] was specified in `flags` and the final component ("basename")
    ///                 of the new file's `pathname` is invalid (e.g., it contains characters not
    ///                 permitted by the underlying filesystem).
    ///  * [`EINVAL`] - The final component ("basename") of `pathname` is invalid (e.g., it
    ///                 contains characters not permitted by the underlying filesystem).
    ///  * [`EISDIR`] - `pathname` refers to a directory and the access requested involved writing
    ///                 (that is, [`O_WRONLY`] or [`O_RDWR`] is set).
    ///  * [`EISDIR`] - `pathname` refers to an existing directory, [`O_TMPFILE`] and one of
    ///                 [`O_WRONLY`] or [`O_RDWR`] were specified in `flags`, but this kernel
    ///                 version does not provide the [`O_TMPFILE`] functionality.
    ///  * [`ELOOP`] - Too many symbolic links were encountered in resolving `pathname`.
    ///  * [`ELOOP`] - `pathname` was a symbolic link, and `flags` specified [`O_NOFOLLOW`] but not
    ///                [`O_PATH`].
    ///  * [`EMFILE`] - The per-process limit on the number of open file descriptors has been
    ///                 reached (see the description of [`RLIMIT_NOFILE`] in [`getrlimit`]).
    ///  * [`ENAMETOOLONG`] - `pathname` was too long.
    ///  * [`ENFILE`] - The system-wide limit on the total number of open files has been reached.
    ///  * [`ENODEV`] - `pathname` refers to a device special file and no corresponding device
    ///                 exists. (This is a Linux kernel bug; in this situation [`ENXIO`] must be
    ///                 returned.)
    ///  * [`ENOENT`] - [`O_CREAT`] is not set and the named file does not exist.
    ///  * [`ENOENT`] - A directory component in `pathname` does not exist or is a dangling
    ///                 symbolic link.
    ///  * [`ENOENT`] - `pathname` refers to a nonexistent directory, [`O_TMPFILE`] and one of
    ///                 [`O_WRONLY`] or [`O_RDWR`] were specified in `flags`, but this kernel
    ///                 version does not provide the [`O_TMPFILE`] functionality.
    ///  * [`ENOMEM`] - The named file is a FIFO, but memory for the FIFO buffer can't be allocated
    ///                 because the per-user hard limit on memory allocation for pipes has been
    ///                 reached and the caller is not privileged.
    ///  * [`ENOMEM`] - Insufficient kernel memory was available.
    ///  * [`ENOSPC`] - `pathname` was to be created but the device containing `pathname` has no
    ///                 room for the new file.
    ///  * [`ENOTDIR`] - A component used as a directory in `pathname` is not, in fact, a
    ///                  directory, or [`O_DIRECTORY`] was specified and `pathname` was not a
    ///                  directory.
    ///  * [`ENXIO`] - [`O_NONBLOCK`] | [`O_WRONLY`] is set, the named file is a FIFO, and no
    ///                process has the FIFO open for reading.
    ///  * [`ENXIO`] - The file is a device special file and no corresponding device exists.
    ///  * [`ENXIO`] - The file is a UNIX domain socket.
    ///  * [`EOPNOTSUPP`] - The filesystem containing `pathname` does not support [`O_TMPFILE`].
    ///  * [`EOVERFLOW`] - `pathname` refers to a regular file that is too large to be opened. The
    ///                    usual scenario here is that an application compiled on a 32-bit platform
    ///                    without `-D_FILE_OFFSET_BITS=64` tried to open a file whose size exceeds
    ///                    `(1<<31)-1` bytes; see also [`O_LARGEFILE`] above. This is the error
    ///                    specified by POSIX.1; before Linux 2.6.24, Linux gave the error
    ///                    [`EFBIG`] for this case.
    ///  * [`EPERM`] - The [`O_NOATIME`] flag was specified, but the effective user ID of the
    ///                caller did not match the owner of the file and the caller was not
    ///                privileged.
    ///  * [`EPERM`] - The operation was prevented by a file seal; see [`fcntl`].
    ///  * [`EROFS`] - `pathname` refers to a file on a read-only filesystem and write access was
    ///                requested.
    ///  * [`ETXTBSY`] - `pathname` refers to an executable image which is currently being executed
    ///                  and write access was requested.
    ///  * [`ETXTBSY`] - `pathname` refers to a file that is currently in use as a swap file, and
    ///                  the [`O_TRUNC`] flag was specified.
    ///  * [`ETXTBSY`] - `pathname` refers to a file that is currently being read by the kernel
    ///                  (e.g., for module/firmware loading), and write access was requested.
    ///  * [`EWOULDBLOCK`] - The [`O_NONBLOCK`] flag was specified, and an incompatible lease was
    ///                      held on the file (see [`fcntl`]).
    pub fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
}
