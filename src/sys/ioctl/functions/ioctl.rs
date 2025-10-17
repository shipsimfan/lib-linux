use std::ffi::{c_int, c_ulong};

// rustdoc imports
#[allow(unused_imports)]
use crate::errno::{errno, EBADF, EFAULT, EINVAL, ENOTTY};

extern "C" {
    /// Control Device
    ///
    /// The [`ioctl`] system call manipulates the underlying device parameters of special files. In
    /// particular, many operating characteristics of character special files (e.g., terminals) may
    /// be controlled with [`ioctl`] operations.  The argument fd must be an open file descriptor.
    ///
    /// The second argument is a device-dependent operation code.  The third argument is an untyped
    /// pointer to memory.  It's traditionally `char *argp` (from the days before `void*` was valid
    /// C), and will be so named for this discussion.
    ///
    /// An [`ioctl`] op has encoded in it whether the argument is an in parameter or out parameter,
    /// and the size of the argument argp in bytes.
    ///
    /// # Return Value
    /// Usually, on success zero is returned. A few [`ioctl`] operations use the return value as an
    /// output parameter and return a nonnegative value on success.  On error, -1 is returned, and
    /// [`errno`] is set to indicate the error.
    ///
    /// # Errors
    ///  * [`EBADF`] - fd is not a valid file descriptor.
    ///  * [`EFAULT`] - argp references an inaccessible memory area.
    ///  * [`EINVAL`] - op or argp is not valid.
    ///  * [`ENOTTY`] - fd is not associated with a character special device.
    ///  * [`ENOTTY`] - The specified operation does not apply to the kind of object that the file descriptor fd references.
    pub fn ioctl(fd: c_int, op: c_ulong, ...) -> c_int;
}
