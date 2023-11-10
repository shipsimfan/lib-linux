use crate::raw::msghdr;
use core::ffi::{c_int, c_ssize_t};

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::{
    errno, MSG_CONFIRM, MSG_DONTROUTE, MSG_DONTWAIT, MSG_EOR, MSG_MORE, MSG_NOSIGNAL, MSG_OOB,
    SOCK_SEQPACKET, SOCK_STREAM,
};

extern "C" {
    /// sendmsg - send a message on a socket
    ///
    /// ## Description
    /// The [`sendmsg`] system call is used to transmit a message to another socket.
    ///
    /// The argument `sockfd` is the file descriptor of the sending socket.
    ///
    /// For [`sendmsg`], the address of the target is given by `msg.name`, with `msg.namelen`
    /// specifying its size.
    ///
    /// For [`sendmsg`], the message is pointed to by the elements of the array `msg.iov`. The
    /// [`sendmsg`] call also allows sending ancillary data (also known as control information).
    ///
    /// If the message is too long to pass atomically through the underlying protocol, the error
    /// [`EMSGSIZE`] is returned, and the message is not transmitted.
    ///
    /// No indication of failure to deliver is implicit in a [`sendmsg`]. Locally detected errors
    /// are indicated by a return value of -1.
    ///
    /// When the message does not fit into the send buffer of the socket, [`sendmsg`] normally
    /// blocks, unless the socket has been placed in nonblocking I/O mode. In nonblocking mode it
    /// would fail with the error [`EAGAIN`] or [`EWOULDBLOCK`] in this case. The [`select`] call
    /// may be used to determine when it is possible to send more data.
    ///
    /// The `flags` argument is the bitwise OR of zero or more of the following flags:
    ///  * [`MSG_CONFIRM`] - Tell the link layer that forward progress happened: you got a
    ///                      successful reply from the other side. If the link layer doesn't get
    ///                      this it will regularly reprobe the neighbor (e.g., via a unicast ARP).
    ///                      Only valid on [`SOCK_DGRAM`] and [`SOCK_RAW`] sockets and currently
    ///                      only implemented for IPv4 and IPv6.
    ///  * [`MSG_DONTROUTE`] - Don't use a gateway to send out the packet, only send to hosts on
    ///                        connected networks. This is usually used only by diagnostic or
    ///                        routing programs. This is only defined for protocol families that
    ///                        route; packet sockets don't.
    ///  * [`MSG_DONTWAIT`] - Enables nonblocking operation; if the operation would block,
    ///                       [`EAGAIN`] or [`EWOULDBLOCK`] is returned (this can also be enabled
    ///                       using the [`O_NONBLOCK`] flag with the [`F_SETFL`] [`fcntl`]).
    ///  * [`MSG_EOR`] - Terminates a record (when this notion is supported, as for sockets of type
    ///                  [`SOCK_SEQPACKET`]).
    ///  * [`MSG_MORE`] - The caller has more data to send. This flag is used with TCP sockets to
    ///                   obtain the same effect as the [`TCP_CORK`] socket option, with the
    ///                   difference that this flag can be set on a per-call basis. Since Linux
    ///                   2.6, this flag is also suported for UDP sockets, and informs the kernel
    ///                   to package all of the data sent in calls with this flag set into a single
    ///                   datagram which is only transmitted when a call is performed that does not
    ///                   specify this flag.
    ///  * [`MSG_NOSIGNAL`] - Requests not to send [`SIGPIPE`] on errors on stream oriented sockets
    ///                       when the other end breaks the connection. The [`EPIPE`] error is
    ///                       still returned.
    ///  * [`MSG_OOB`] - Sends out-of-band data on sockets that support this notion (e.g., of type
    ///                  [`SOCK_STREAM`]); the underlying protocol must also support out-of-band
    ///                  data.
    ///
    /// ## Return Value
    /// On success, these calls return the number of characters sent. On error, -1 is returned, and
    /// [`errno`] is set appropriately.
    pub fn sendmsg(sockfd: c_int, msg: *const msghdr, flags: c_int) -> c_ssize_t;
}
