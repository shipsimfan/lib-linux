use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::errno,
    sys::socket::{SOCK_SEQPACKET, SOCK_STREAM},
};

extern "C" {
    /// listen - listen for connections on a socket
    ///
    /// [`listen`] marks the socket referred to by `sockfd` as a passive socket, that is, as a
    /// socket that will be used to accept incoming connection requests using [`accept`].
    ///
    /// The `sockfd` argument is a file descriptor that refers to a socket of type [`SOCK_STREAM`]
    /// or [`SOCK_SEQPACKET`].
    ///
    /// The `backlog` argument defines the maximum length to which the queue of pending connections
    /// for `sockfd` may grow. If a connection request arrives when the queue is full, the client
    /// may receive an error with an indication of [`ECONNREFUSED`] or, if the underlying protocol
    /// supports retransmission, the request may be ignored so that a later reattempt at connection
    /// succeeds.
    ///
    /// # Return Value
    /// On success, zero is returned. On error, -1 is returned, and [`errno`] is set to indicate
    /// the error.
    pub fn listen(sockfd: c_int, backlog: c_int) -> c_int;
}
