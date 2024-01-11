use crate::sys::socket::msghdr;
use core::ffi::{c_int, c_ssize_t};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::errno,
    sys::socket::{
        MSG_CMSG_CLOEXEC, MSG_DONTWAIT, MSG_ERRQUEUE, MSG_OOB, MSG_PEEK, MSG_TRUNC, MSG_WAITALL,
    },
};
#[allow(unused_imports)]
use std::ptr::null;

extern "C" {
    /// recvmsg - receive a message from a socket
    ///
    /// ## Description
    /// The [`recvmsg`] call is used to receive messages from a socket, and may be used to receive
    /// data on a socket whether or not it is connection-oriented.
    ///
    /// If `src_addr` is not [`null`], and the underlying protocol provides the source address,
    /// this source address is filled in. When `src_addr` is [`null`], nothing is filled in; in
    /// this case, addrlen is not used, and should also be [`null`]. The argument addrlen is a
    /// value-result argument, which the caller should initialize before the call to the size of
    /// the buffer associated with `src_addr`, and modified on return to indicate the actual size
    /// of the source address. The returned address is truncated if the buffer provided is too
    /// small; in this case, `addrlen` will return a value greater than was supplied to the call.
    ///
    /// [`recvmsg`] returns the length of the message on successful completion. If a message is
    /// too long to fit in the supplied buffer, excess bytes may be discarded depending on the type
    /// of socket the message is received from.
    ///
    /// If no messages are available at the socket, the receive calls wait for a message to arrive,
    /// unless the socket is nonblocking (see [`fcntl`]), in which case the value -1 is returned
    /// and the external variable [`errno`] is set to [`EAGAIN`] or [`EWOULDBLOCK`]. The receive
    /// calls normally return any data available, up to the requested amount, rather than waiting
    /// for receipt of the full amount requested.
    ///
    /// The [`select`] or [`poll`] call may be used to determine when more data arrives.
    ///
    /// The flags argument to a [`recvmsg`] call is formed by ORing one or more of the following
    /// values:
    ///  * [`MSG_CMSG_CLOEXEC`] - Set the close-on-exec flag for the file descriptor received via a
    ///                           UNIX domain file descriptor using the `SCM_RIGHTS` operation.
    ///                           This flag is useful for the same reasons as the [`O_CLOEXEC`]
    ///                           flag of [`open`].
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
    /// ## Return Value
    /// These calls return the number of bytes received, or -1 if an error occurred. The return
    /// value will be 0 when the peer has performed an orderly shutdown.
    pub fn recvmsg(sockfd: c_int, msg: *mut msghdr, flags: c_int) -> c_ssize_t;
}
