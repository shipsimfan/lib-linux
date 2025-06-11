use crate::{c_size_t, c_ssize_t};
use core::ffi::{c_int, c_void};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{
        errno, EAGAIN, EBADF, ECONNREFUSED, EFAULT, EINTR, EINVAL, ENOTCONN, ENOTSOCK, EWOULDBLOCK,
    },
    sys::socket::{
        accept, connect, recvfrom, sockaddr, AF_PACKET, AF_UNSPEC, MSG_DONTWAIT, MSG_ERRQUEUE,
        MSG_OOB, MSG_PEEK, MSG_TRUNC, MSG_WAITALL,
    },
};
#[allow(unused_imports)]
use std::ptr::null;

extern "C" {
    /// The [`recv`] call is normally used only on a connected socket (see [`connect`]) and is
    /// identical to [`recvfrom`] with a [`null`] `src_addr` argument.
    ///
    /// [`recv`] returns the length of the message on successful completion. If a message is too
    /// long to fit in the supplied buffer, excess bytes may be discarded depending on the type of
    /// socket the message is received from.
    ///
    /// If no messages are available at the socket, the receive calls wait for a message to arrive,
    /// unless the socket is nonblocking (see [`fcntl`]), in which case the value -1 is returned
    /// and the external variable [`errno`] is set to [`EAGAIN`] or [`EWOULDBLOCK`]. The receive
    /// calls normally return any data available, up to the requested amount, rather than waiting
    /// for receipt of the full amount requested.
    ///
    /// The [`select`] or [`poll`] call may be used to determine when more data arrives.
    ///
    /// The flags argument to a [`recv`] call is formed by ORing one or more of the following
    /// values:
    ///  * [`MSG_DONTWAIT`] - Enables nonblocking operation; if the operation would block, the call
    ///                       fails with the error [`EAGAIN`] or [`EWOULDBLOCK`] (this can also be
    ///                       enabled using the [`O_NONBLOCK`] flag with the [`F_SETFL`]
    ///                       [`fcntl`]).
    ///  * [`MSG_ERRQUEUE`] - This flag specifies that queued errors should be received from the
    ///                       socket error queue. The error is passed in an ancillary message with
    ///                       a type dependent on the protocol (for IPv4 [`IP_RECVERR`]). The user
    ///                       should supply a buffer of sufficient size. The payload of the
    ///                       original packet that caused the error is passed as normal data via
    ///                       `iovec`. The original destination address of the datagram that caused
    ///                       the error is supplied via msg_name. For local errors, no address is
    ///                       passed (this can be checked with the cmsg_len member of the cmsghdr).
    ///                       For error receives, the [`MSG_ERRQUEUE`] is set in the msghdr. After
    ///                       an error has been passed, the pending socket error is regenerated
    ///                       based on the next queued error and will be passed on the next socket
    ///                       operation. The error is supplied in a `sock_extended_err` structure.
    ///                       `ee_errno` contains the `errno` number of the queued error.
    ///                       `ee_origin` is the origin code of where the error originated. The
    ///                       other fields are protocol-specific. The macro [`SOCK_EE_OFFENDER`]
    ///                       returns a pointer to the address of the network object where the
    ///                       error originated from given a pointer to the ancillary message. If
    ///                       this address is not known, the `sa_family` member of the [`sockaddr`]
    ///                       contains [`AF_UNSPEC`] and the other fields of the sockaddr are
    ///                       undefined. The payload of the packet that caused the error is passed
    ///                       as normal data. For local errors, no address is passed (this can be
    ///                       checked with the cmsg_len member of the cmsghdr). For error receives,
    ///                       the [`MSG_ERRQUEUE`] is set in the `msghdr`. After an error has been
    ///                       passed, the pending socket error is regenerated based on the next
    ///                       queued error and will be passed on the next socket operation.
    ///  * [`MSG_OOB`] - This flag requests receipt of out-of-band data that would not be received
    ///                  in the normal data stream. Some protocols place expedited data at the head
    ///                  of the normal data queue, and thus this flag cannot be used with such
    ///                  protocols.
    ///  * [`MSG_PEEK`] - This flag causes the receive operation to return data from the beginning
    ///                   of the receive queue without removing that data from the queue. Thus, a
    ///                   subsequent receive call will return the same data.
    ///  * [`MSG_TRUNC`] - For raw ([`AF_PACKET`]), Internet datagram, netlink and UNIX datagram
    ///                    sockets: return the real length of the packet or datagram, even when it
    ///                    was longer than the passed buffer. Not implemented for UNIX domain
    ///                    sockets. For use with Internet stream sockets.
    ///  * [`MSG_WAITALL`] - This flag requests that the operation block until the full request is
    ///                      satisfied. However, the call may still return less data than requested
    ///                      if a signal is caught, an error or disconnect occurs, or the next data
    ///                      to be received is of a different type than that returned.
    ///
    /// # Return Value
    /// These calls return the number of bytes received, or -1 if an error occurred. The return
    /// value will be 0 when the peer has performed an orderly shutdown.
    ///
    /// # Errors
    /// These are some standard errors generated by the socket layer. Additional errors may be
    /// generated and returned from the underlying protocol modules; see their manual pages.
    ///  * [`EAGAIN`] or [`EWOULDBLOCK`] - The socket is marked nonblocking and the receive
    ///                                    operation would block, or a receive timeout had been set
    ///                                    and the timeout expired before data was received.
    ///                                    POSIX.1 allows either error to be returned for this
    ///                                    case, and does not require these constants to have the
    ///                                    same value, so a portable application should check for
    ///                                    both possibilities.   
    ///  * [`EBADF`] - The argument `sockfd` is an invalid file descriptor.
    ///  * [`ECONNREFUSED`] - A remote host refused to allow the network connection (typically
    ///                       because it is not running the requested service).
    ///  * [`EFAULT`] - The receive buffer pointer(s) point outside the process's address space.
    ///  * [`EINTR`] - The receive was interrupted by delivery of a signal before any data was
    ///                available.
    ///  * [`EINVAL`] - Invalid argument passed.
    ///  * [`ENOTCONN`] - The socket is associated with a connection-oriented protocol and has not
    ///                   been connected (see [`connect`] and [`accept`]).
    ///  * [`ENOTSOCK`] - The file descriptor `sockfd` does not refer to a socket.
    pub fn recv(sockfd: c_int, buf: *mut c_void, len: c_size_t, flags: c_int) -> c_ssize_t;
}
