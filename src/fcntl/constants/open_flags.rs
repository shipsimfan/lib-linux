use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{errno::ELOOP, unistd::read};

/// The mask of access mode flags
pub const O_ACCMODE: c_int = 0o003;

/// Read only access
pub const O_RDONLY: c_int = 0;

/// Write only access
pub const O_WRONLY: c_int = 1;

/// Read and write access
pub const O_RDWR: c_int = 2;

/// If `pathname` does not exist, create it as a regular file
pub const O_CREAT: c_int = 0o100;

/// Ensure that this call creates the file
pub const O_EXCL: c_int = 0o200;

/// If `pathname` refers to a terminal device, it will not become the process's controlling
/// terminal even if the process does not have one
pub const O_NOCTTY: c_int = 0o400;

/// If the file already exists and is a regular file and the access mode allows writing, it will be
/// truncated to length 0
pub const O_TRUNC: c_int = 0o01000;

/// The file is opened in append mode
pub const O_APPEND: c_int = 0o02000;

/// When possible, the file is opened in nonblocking mode
pub const O_NONBLOCK: c_int = 0o4000;

/// When possible, the file is opened in nonblocking mode
pub const O_NDELAY: c_int = O_NONBLOCK;

/// Write operations on the file will complete according to the requirements of synchronized I/O
/// data integrity completion
pub const O_DSYNC: c_int = 0o10000;

/// Enable signal-driven I/O
pub const O_ASYNC: c_int = 0o20000;

/// Try to minimize cache effects of the I/O to and from this file
pub const O_DIRECT: c_int = 0o40000;

/// Allow files whose sizes cannot be represented in an [`off_t`] (but can be represented in an
/// [`off64_t`]) to be opened
pub const O_LARGEFILE: c_int = 0o100000;

/// If `pathname` is not a directory, cause the open to fail
pub const O_DIRECTORY: c_int = 0o200000;

/// If the trailing component (i.e., basename) of `pathname` is a symbolic link, then the open
/// fails, with the error [`ELOOP`]
pub const O_NOFOLLOW: c_int = 0o400000;

/// Do not update the file last access time when the file is [`read`]
pub const O_NOATIME: c_int = 0o1000000;

/// Enable the close-on-exec flag for the new file descriptor
pub const O_CLOEXEC: c_int = 0o2000000;

/// Write operations on the file will complete according to the requirements of synchronized I/O
/// file integrity completion (by contrast with the synchronized I/O data integrity completion
/// provided by [`O_DSYNC`]
pub const O_SYNC: c_int = 0o4010000;

/// Obtain a file descriptor that can be used for two purposes: to indicate a location in the
/// filesystem tree and to perform operations that act purely at the file descriptor level
pub const O_PATH: c_int = 0o10000000;

/// Create an unnamed temporary regular file
pub const O_TMPFILE: c_int = 0o20000000 | O_DIRECTORY;
