use crate::sys::socket::{sockaddr, socklen_t};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::errno::{errno, EBADF, EINVAL, ENOBUFS, ENOTCONN, ENOTSOCK, EOPNOTSUPP};

extern "C" {
    /// Get the name of the peer socket
    ///
    /// # Description
    /// The [`getpeername`] function shall retrieve the peer address of the specified `socket`,
    /// store this address in the [`sockaddr`] structure pointed to by the `address` argument, and
    /// store the length of this address in the object pointed to by the `address_len` argument.
    ///
    /// If the actual length of the address is greater than the length of the supplied [`sockaddr`]
    /// structure, the stored address shall be truncated.
    ///
    /// If the protocol permits connections by unbound clients, and the peer is not bound, then the
    /// value stored in the object pointed to by address is unspecified.
    ///
    /// # Return Value
    /// Upon successful completion, 0 shall be returned. Otherwise, -1 shall be returned and
    /// [`errno`] set to indicate the error.
    ///
    /// # Errors
    /// The [`getpeername`] function shall fail if:
    ///  * [`EBADF`] - The `socket` argument is not a valid file descriptor.
    ///  * [`EINVAL`] - The socket has been shut down.
    ///  * [`ENOTCONN`] - The socket is not connected or otherwise has not had the peer
    ///                   pre-specified.
    ///  * [`ENOTSOCK`] - The `socket` argument does not refer to a socket.
    ///  * [`EOPNOTSUPP`] - The operation is not supported for the socket protocol.
    ///  * [`ENOBUFS`] - Insufficient resources were available in the system to complete the call.
    pub fn getpeername(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t)
        -> c_int;
}
