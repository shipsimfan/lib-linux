use crate::sys::socket::{sockaddr, socklen_t};
use std::ffi::c_int;

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
    pub fn getsockname(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t)
        -> c_int;
}
