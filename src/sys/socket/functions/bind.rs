use crate::sys::socket::{sockaddr, socklen_t};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{
        errno, EACCES, EADDRINUSE, EADDRNOTAVAIL, EBADF, EFAULT, EINVAL, ELOOP, ENAMETOOLONG,
        ENOENT, ENOMEM, ENOTDIR, ENOTSOCK, EROFS,
    },
    sys::socket::{accept, socket, SOCK_STREAM},
};

extern "C" {
    /// When a socket is created with [`socket`], it exists in a name space (address family) but
    /// has no address assigned to it. [`bind`] assigns the address specified by `addr` to the
    /// socket referred to by the file descriptor `sockfd`. `addrlen` specifies the size, in bytes,
    /// of the address structure pointed to by `addr`. Traditionally, this operation is called
    /// “assigning a name to a socket”.
    ///
    /// It is normally necessary to assign a local address using [`bind`] before a [`SOCK_STREAM`]
    /// socket may receive connections (see [`accept`]).
    ///
    /// The rules used in name binding vary between address families. The actual structure passed
    /// for the addr argument will depend on the address family.
    ///
    /// # Return Value
    /// On success, zero is returned. On error, -1 is returned, and [`errno`] is set to indicate
    /// the error.
    ///
    /// # Errors
    ///  * [`EACCES`] - The address is protected, and the user is not the superuser.
    ///  * [`EADDRINUSE`] - The given address is already in use.
    ///  * [`EADDRINUSE`] - (Internet domain sockets) The port number was specified as zero in the
    ///                     socket address structure, but, upon attempting to bind to an ephemeral
    ///                     port, it was determined that all port numbers in the ephemeral port
    ///                     range are currently in use.
    ///  * [`EBADF`] - `sockfd` is not a valid file descriptor.
    ///  * [`EINVAL`] - The socket is already bound to an address.
    ///  * [`EINVAL`] - `addrlen` is wrong, or `addr` is not a valid address for this socket's
    ///                 domain.
    ///  * [`ENOTSOCK`] - The file descriptor `sockfd` does not refer to a socket.
    ///
    /// The following errors are specific to UNIX domain (AF_UNIX) sockets:
    ///  * [`EACCES`] - Search permission is denied on a component of the path prefix.
    ///  * [`EADDRNOTAVAIL`] - A nonexistent interface was requested or the requested address was
    ///                        not local.
    ///  * [`EFAULT`] - `addr` points outside the user's accessible address space.
    ///  * [`ELOOP`] - Too many symbolic links were encountered in resolving `addr`.
    ///  * [`ENAMETOOLONG`] - `addr` is too long.
    ///  * [`ENOENT`] - A component in the directory prefix of the socket pathname does not exist.
    ///  * [`ENOMEM`] - Insufficient kernel memory was available.
    ///  * [`ENOTDIR`] - A component of the path prefix is not a directory.
    ///  * [`EROFS`] - The socket inode would reside on a read-only filesystem.
    pub fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
}
