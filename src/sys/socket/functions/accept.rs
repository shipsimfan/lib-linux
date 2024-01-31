use crate::sys::socket::{sockaddr, socklen_t};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{
        errno, EAGAIN, EBADF, ECONNABORTED, EFAULT, EHOSTDOWN, EHOSTUNREACH, EINTR, EINVAL, EMFILE,
        ENETDOWN, ENETUNREACH, ENFILE, ENOBUFS, ENOMEM, ENONET, ENOPROTOOPT, ENOTSOCK, EOPNOTSUPP,
        EPERM, EPROTO, EWOULDBLOCK,
    },
    signal::SIGIO,
    sys::socket::{bind, listen, socket, SOCK_SEQPACKET, SOCK_STREAM},
};
#[allow(unused_imports)]
use std::ptr::null_mut;

extern "C" {
    /// The [`accept`] system call is used with connection-based socket types ([`SOCK_STREAM`],
    /// [`SOCK_SEQPACKET`]). It extracts the first connection request on the queue of pending
    /// connections for the listening socket, `sockfd`, creates a new connected socket, and returns
    /// a new file descriptor referring to that socket. The newly created socket is not in the
    /// listening state. The original socket `sockfd` is unaffected by this call.
    ///
    /// The argument `sockfd` is a socket that has been created with [`socket`], bound to a local
    /// address with [`bind`], and is listening for connections after a [`listen`].
    ///
    /// The argument `addr` is a pointer to a [`sockaddr`] structure.  This structure is filled in
    /// with the address of the peer socket, as known to the communications layer. The exact format
    /// of the address returned `addr` is determined by the socket's address family (see [`socket`]
    /// and the respective protocol man pages). When `addr` is [`null_mut`], nothing is filled in;
    /// in this case, `addrlen` is not used, and should also be 0.
    ///
    /// The `addrlen` argument is a value-result argument: the caller must initialize it to contain
    /// the size (in bytes) of the structure pointed to by `addr`; on return it will contain the
    /// actual size of the peer address.
    ///
    /// The returned address is truncated if the buffer provided is too small; in this case,
    /// `addrlen` will return a value greater than was supplied to the call.
    ///
    /// If no pending connections are present on the queue, and the socket is not marked as
    /// nonblocking, [`accept`] blocks the caller until a connection is present. If the socket is
    /// marked nonblocking and no pending connections are present on the queue, [`accept`] fails
    /// with the error [`EAGAIN`] or [`EWOULDBLOCK`].
    ///
    /// In order to be notified of incoming connections on a socket, you can use [`select`],
    /// [`poll`], or [`epoll`]. A readable event will be delivered when a new connection is
    /// attempted and you may then call [`accept`] to get a socket for that connection.
    /// Alternatively, you can set the socket to deliver [`SIGIO`] when activity occurs on a
    /// socket.
    ///
    /// # Return Value
    ///  On success, these system calls return a file descriptor for the accepted socket (a
    /// nonnegative integer). On error, -1 is returned, [`errno`] is set to indicate the error, and
    /// `addrlen` is left unchanged.
    ///
    /// ## Error Handling
    /// Linux [`accept`] passes already-pending network errors on the new socket as an error code
    /// from [`accept`]. This behavior differs from other BSD socket implementations. For reliable
    /// operation the application should detect the network errors defined for the protocol after
    /// [`accept`] and treat them like [`EAGAIN`] by retrying. In the case of TCP/IP, these are
    /// [`ENETDOWN`], [`EPROTO`], [`ENOPROTOOPT`], [`EHOSTDOWN`], [`ENONET`], [`EHOSTUNREACH`],
    /// [`EOPNOTSUPP`], and [`ENETUNREACH`].
    ///
    /// # Errors
    ///  * [`EAGAIN`] or [`EWOULDBLOCK`] - The socket is marked nonblocking and no connections are
    ///                                    present to be accepted. POSIX.1-2001 and POSIX.1-2008
    ///                                    allow either error to be returned for this case, and do
    ///                                    not require these constants to have the same value, so a
    ///                                    portable application should check for both
    ///                                    possibilities.
    ///  * [`EBADF`] -  `sockfd` is not an open file descriptor.
    ///  * [`ECONNABORTED`] - A connection has been aborted.
    ///  * [`EFAULT`] - The `addr` argument is not in a writable part of the user address space.
    ///  * [`EINTR`] -  The system call was interrupted by a signal that was caught before a valid
    ///                 connection arrived.
    ///  * [`EINVAL`] - Socket is not listening for connections, or `addrlen` is invalid (e.g., is
    ///                 negative).
    ///  * [`EMFILE`] - The per-process limit on the number of open file descriptors has been
    ///                 reached.
    ///  * [`ENFILE`] - The system-wide limit on the total number of open files has been reached.
    ///  * [`ENOBUFS`] or [`ENOMEM`] - Not enough free memory. This often means that the memory
    ///                                allocation is limited by the socket buffer limits, not by
    ///                                the system memory.
    ///  * [`ENOTSOCK`] - The file descriptor `sockfd` does not refer to a socket.
    ///  * [`EOPNOTSUPP`] - The referenced socket is not of type [`SOCK_STREAM`].
    ///  * [`EPERM`] -  Firewall rules forbid connection.
    ///  * [`EPROTO`] - Protocol error.
    pub fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: socklen_t) -> c_int;
}
