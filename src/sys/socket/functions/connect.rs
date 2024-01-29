use crate::sys::socket::{sockaddr, socklen_t};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{
        errno, EACCES, EADDRINUSE, EADDRNOTAVAIL, EAFNOSUPPORT, EAGAIN, EALREADY, EBADF,
        ECONNREFUSED, EFAULT, EINPROGRESS, EINTR, EISCONN, ENETUNREACH, ENOTSOCK, EPERM,
        EPROTOTYPE, ETIMEDOUT,
    },
    sys::socket::{AF_UNSPEC, SOCK_DGRAM, SOCK_SEQPACKET, SOCK_STREAM},
};

extern "C" {
    /// The [`connect`] system call connects the socket referred to by the file descriptor `sockfd`
    /// to the address specified by `addr`. The `addrlen` argument specifies the size of `addr`.
    /// The format of the address in `addr` is determined by the address space of the socket
    /// `sockfd`.
    ///
    /// If the socket `sockfd` is of type [`SOCK_DGRAM`], then `addr` is the address to which
    /// datagrams are sent by default, and the only address from which datagrams are received. If
    /// the socket is of type [`SOCK_STREAM`] or [`SOCK_SEQPACKET`], this call attempts to make a
    /// connection to the socket that is bound to the address specified by `addr`.
    ///
    /// Some protocol sockets (e.g., UNIX domain stream sockets) may successfully [`connect`] only
    /// once.
    ///
    /// Some protocol sockets (e.g., datagram sockets in the UNIX and Internet domains) may use
    /// [`connect`] multiple times to change their association.
    ///
    /// Some protocol sockets (e.g., TCP sockets as well as datagram sockets in the UNIX and
    /// Internet domains) may dissolve the association by connecting to an address with the
    /// `family` member of [`sockaddr`] set to [`AF_UNSPEC`]; thereafter, the socket can be
    /// connected to another address. ([`AF_UNSPEC`] is supported since Linux 2.2.)
    ///
    /// # Return Value
    /// If the connection or binding succeeds, zero is returned. On error, -1 is returned, and
    /// [`errno`] is set to indicate the error.
    ///
    /// # Errors
    /// The following are general socket errors only. There may be other domain-specific error
    /// codes.
    ///  * [`EACCES`] - For UNIX domain sockets, which are identified by pathname: Write permission
    ///                 is denied on the socket file, or search  permission is denied for one of
    ///                 the directories in the path prefix.
    ///  * [`EACCES`] or [`EPERM`] - The user tried to connect to a broadcast address without
    ///                              having the socket broadcast flag enabled or the connection
    ///                              request failed because of a local firewall rule.
    ///  * [`EACCES`] - It can also be returned if an SELinux policy denied a connection (for
    ///                 example, if there is a policy saying that an HTTP proxy can only connect to
    ///                 ports associated with HTTP servers, and the proxy tries to connect to a
    ///                 different port).
    ///  * [`EADDRINUSE`] - Local address is already in use.
    ///  * [`EADDRNOTAVAIL`] - (Internet domain sockets) The socket referred to by `sockfd` had not
    ///                        previously been bound to an address and, upon attempting to bind it
    ///                        to an ephemeral port, it was determined that all port numbers in the
    ///                        ephemeral port range are currently in use.
    ///  * [`EAFNOSUPPORT`] - The passed address didn't have the correct address family in its
    ///                       `family` field.
    ///  * [`EAGAIN`] - For nonblocking UNIX domain sockets, the socket is nonblocking, and the
    ///                 connection cannot be completed immediately. For other socket families,
    ///                 there are insufficient entries in the routing cache.
    ///  * [`EALREADY`] - The socket is nonblocking and a previous connection attempt has not yet
    ///                   been completed.
    ///  * [`EBADF`] - `sockfd` is not a valid open file descriptor.
    ///  * [`ECONNREFUSED`] - A [`connect`] on a stream socket found no one listening on the remote
    ///                       address.
    ///  * [`EFAULT`] - The socket structure address is outside the user's address space.
    ///  * [`EINPROGRESS`] - The socket is nonblocking and the connection cannot be completed
    ///                      immediately. (UNIX domain sockets failed with [`EAGAIN`] instead.) It
    ///                      is possible to [`select`] or [`poll`] for completion by selecting the
    ///                      socket for writing. After [`select`] indicates writability, use
    ///                      [`getsockopt`] to read the [`SO_ERROR`] option at level [`SOL_SOCKET`]
    ///                      to determine whether [`connect`] completed successfully ([`SO_ERROR`]
    ///                      is zero) or unsuccessfully ([`SO_ERROR`] is one of the usual error
    ///                      codes listed here, explaining the reason for the failure).
    ///  * [`EINTR`] - The system call was interrupted by a signal that was caught.
    ///  * [`EISCONN`] - The socket is already connected.
    ///  * [`ENETUNREACH`] - Network is unreachable.
    ///  * [`ENOTSOCK`] - The file descriptor `sockfd` does not refer to a socket.
    ///  * [`EPROTOTYPE`] - The socket type does not support the requested communications protocol.
    ///                     This error can occur, for example, on an attempt to connect a UNIX
    ///                     domain datagram socket to a stream socket.
    ///  * [`ETIMEDOUT`] - Timeout while attempting connection. The server may be too busy to
    ///                    accept new connections. Note that for IP sockets the timeout may be very
    ///                    long when syncookies are enabled on the server.
    pub fn connect(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
}
