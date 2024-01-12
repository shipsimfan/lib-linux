use crate::sys::socket::{sockaddr, socklen_t};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::errno,
    sys::socket::{socket, SOCK_STREAM},
};

extern "C" {
    /// bind - bind a name to a socket
    ///
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
    pub fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
}
