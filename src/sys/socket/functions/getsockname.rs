use crate::sys::socket::{sockaddr, socklen_t};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::errno::{errno, EBADF, EINVAL, ENOBUFS, ENOTSOCK, EOPNOTSUPP};

extern "C" {
    /// Get the socket name
    ///
    /// # Description
    /// The [`getsockname`] function shall retrieve the locally-bound name of the specified
    /// `socket`, store this address in the [`sockaddr`] structure pointed to by the `address`
    /// argument, and store the length of this address in the object pointed to by the
    /// `address_len` argument.
    ///
    /// If the actual length of the address is greater than the length of the supplied [`sockaddr`]
    /// structure, the stored address shall be truncated.
    ///
    /// If the socket has not been bound to a local name, the value stored in the object pointed to
    /// by address is unspecified.
    ///
    /// # Return Value
    /// Upon successful completion, 0 shall be returned, the `address` argument shall point to the
    /// address of the socket, and the `address_len` argument shall point to the length of the
    /// address. Otherwise, -1 shall be returned and [`errno`] set to indicate the error.
    ///
    /// # Errors
    /// The [`getsockname`] function shall fail if:
    ///  * [`EBADF`] - The `socket` argument is not a valid file descriptor.
    ///  * [`ENOTSOCK`] - The `socket` argument does not refer to a socket.
    ///  * [`EOPNOTSUPP`] - The operation is not supported for this socket's protocol.
    ///  * [`EINVAL`] - The socket has been shutdown.
    ///  * [`ENOBUFS`] - Insufficient resources were available in the system to complete the
    ///                  function.
    pub fn getsockname(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t)
        -> c_int;
}
