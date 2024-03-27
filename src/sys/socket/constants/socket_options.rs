use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::ENOPROTOOPT,
    sys::{
        epoll::epoll_wait,
        socket::{
            accept, bind, getsockopt, recvmsg, send, sendmsg, AF_INET, AF_INET6, MSG_OOB,
            SOCK_STREAM,
        },
    },
    unistd::{close, read},
};

/// Socket level options
pub const SOL_SOCKET: c_int = 1;

/// Enable socket debugging
pub const SO_DEBUG: c_int = 1;

/// Indicates that the rules used in validating addresses supplied in a [`bind`] call should allow
/// reuse of local addresses. For [`AF_INET`] sockets this means that a socket may bind, except
/// when there is an active listening socket bound to the address. When the listening socket is
/// bound to [`INADDR_ANY`] with a specific port then it is not possible to bind to this port for
/// any local address. Argument is an integer boolean flag.
pub const SO_REUSEADDR: c_int = 2;

/// Gets the socket type as an integer (e.g., [`SOCK_STREAM`]). This socket option is read-only.
pub const SO_TYPE: c_int = 3;

/// Get and clear the pending socket error. This socket option is read-only. Expects an integer.
pub const SO_ERROR: c_int = 4;

/// Don't send via a gateway, send only to directly connectedhosts. The same effect can be achieved
/// by setting the [`MSG_DONTROUTE`] flag on a socket [`send`] operation.  Expects an integer
/// boolean flag.
pub const SO_DONTROUTE: c_int = 5;

/// Set or get the broadcast flag. When enabled, datagram sockets are allowed to send packets to a
/// broadcast address. This option has no effect on stream-oriented sockets.
pub const SO_BROADCAST: c_int = 6;

/// Sets or gets the maximum socket send buffer in bytes. The kernel doubles this value (to allow
/// space for bookkeeping overhead) when it is set using [`setsockopt`], and this doubled value is
/// returned by [`getsockopt`]. The default value is set by the `/proc/sys/net/core/wmem_default`
/// file and the maximum allowed value is set by the `/proc/sys/net/core/wmem_max file`. The
/// minimum (doubled) value for this option is 2048.
pub const SO_SNDBUF: c_int = 7;

/// Sets or gets the maximum socket receive buffer in bytes. The kernel doubles this value (to
/// allow space for bookkeeping overhead) when it is set using [`setsockopt`], and this doubled
/// value is returned by [`getsockopt`]. The default value is set by the
/// `/proc/sys/net/core/rmem_default` file, and the maximum allowed value is set by the
/// `/proc/sys/net/core/rmem_max file`. The minimum (doubled) value for this option is 256.
pub const SO_RCVBUF: c_int = 8;

/// Using this socket option, a privileged (`CAP_NET_ADMIN`) process can perform the same task as
/// [`SO_SNDBUF`], but the `wmem_max` limit can be overridden.
pub const SO_SNDBUFFORCE: c_int = 32;

/// Using this socket option, a privileged (`CAP_NET_ADMIN`) process can perform the same task as
/// [`SO_RCVBUF`], but the `rmem_max` limit can be overridden.
pub const SO_RCVBUFFORCE: c_int = 33;

/// Enable sending of keep-alive messages on connection-oriented sockets. Expects an integer
/// boolean flag.
pub const SO_KEEPALIVE: c_int = 9;

/// If this option is enabled, out-of-band data is directly placed into the receive data stream.
/// Otherwise, out-of- band data is passed only when the [`MSG_OOB`] flag is set during receiving.
pub const SO_OOBINLINE: c_int = 10;

/// Set the protocol-defined priority for all packets to be sent on this socket. Linux uses this
/// value to order the networking queues: packets with a higher priority may be processed first
/// depending on the selected device queueing discipline. Setting a priority outside the range 0 to
/// 6 requires the `CAP_NET_ADMIN` capability.
pub const SO_PRIORITY: c_int = 12;

/// Sets or gets the [`SO_LINGER`] option. The argument is a linger structure. When enabled, a
/// [`close`] or [`shutdown`] will not return until all queued messages for the socket have been
/// successfully sent or the linger timeout has been reached. Otherwise, the call returns
/// immediately and the closing is done in the background. When the socket is closed as part of
/// [`exit`], it always lingers in the background.
pub const SO_LINGER: c_int = 13;

/// Enable BSD bug-to-bug compatibility. This is used by the UDP protocol module in Linux 2.0 and
/// 2.2. If enabled, ICMP errors received for a UDP socket will not be passed to the user program.
/// In later kernel versions, support for this option has been phased out: Linux 2.4 silently
/// ignores it, and Linux 2.6 generates a kernel warning (printk()) if a program uses this option.
/// Linux 2.0 also enabled BSD bug-to-bug compatibility options (random header changing, skipping
/// of the broadcast flag) for raw sockets with this option, but that was removed in Linux 2.2.
pub const SO_BSDCOMPAT: c_int = 14;

/// Permits multiple [`AF_INET`] or [`AF_INET6`] sockets to be bound to an identical socket
/// address. This option must be set on each socket (including the first socket) prior to calling
/// [`bind`] on the socket. To prevent port hijacking, all of the processes binding to the same
/// address must have the same effective UID. This option can be employed with both TCP and UDP
/// sockets.
///
/// For TCP sockets, this option allows [`accept`] load distribution in a multi-threaded server to
/// be improved by using a distinct listener socket for each thread. This provides improved load
/// distribution as compared to traditional techniques such using a single [`accept`]ing thread
/// that distributes connections, or having multiple threads that compete to [`accept`] from the
/// same socket.
///
/// For UDP sockets, the use of this option can provide better distribution of incoming datagrams
/// to multiple processes (or threads) as compared to the traditional technique of having multiple
/// processes compete to receive datagrams on the same socket.
pub const SO_REUSEPORT: c_int = 15;

/// Enable or disable the receiving of the `SCM_CREDENTIALS` control message.
pub const SO_PASSCRED: c_int = 16;

/// Return the credentials of the peer process connected to this socket.
pub const SO_PEERCRED: c_int = 17;

/// Specify the minimum number of bytes in the buffer until the socket layer will pass the data to
/// the user on receiving. This is initialized to 1. SO_RCVLOWAT is changeable only since Linux
/// 2.4.
pub const SO_RCVLOWAT: c_int = 18;

/// Specify the minimum number of bytes in the buffer until the socket layer will pass the data to
/// the protocol. This is initialized to 1. [`SO_SNDLOWAT`] is not changeable on Linux
/// ([`setsockopt`] fails with the error [`ENOPROTOOPT`]).
pub const SO_SNDLOWAT: c_int = 19;

/// Specify the receiving timeout until reporting an error. The argument is a struct [`timeval`].
/// If an input or output function blocks for this period of time, and data has been sent or
/// received, the return value of that function will be the amount of data transferred; if no data
/// has been transferred and the timeout has been reached, then -1 is returned with [`errno`] set
/// to [`EAGAIN`] or [`EWOULDBLOCK`], or [`EINPROGRESS`] (for [`connect`]) just as if the socket
/// was specified to be nonblocking. If the timeout is set to zero (the default), then the
/// operation will never timeout. Timeouts only have effect for system calls that perform socket
/// I/O (e.g., [`accept`], [`connect`], [`read`], [`recvmsg`], [`send`], [`sendmsg`]); timeouts
/// have no effect for [`select`], [`poll`], [`epoll_wait`], and so on.
pub const SO_RCVTIMEO: c_int = 20;

/// Specify the sending timeout until reporting an error. The argument is a struct [`timeval`].
/// If an input or output function blocks for this period of time, and data has been sent or
/// received, the return value of that function will be the amount of data transferred; if no data
/// has been transferred and the timeout has been reached, then -1 is returned with [`errno`] set
/// to [`EAGAIN`] or [`EWOULDBLOCK`], or [`EINPROGRESS`] (for [`connect`]) just as if the socket
/// was specified to be nonblocking. If the timeout is set to zero (the default), then the
/// operation will never timeout. Timeouts only have effect for system calls that perform socket
/// I/O (e.g., [`accept`], [`connect`], [`read`], [`recvmsg`], [`send`], [`sendmsg`]); timeouts
/// have no effect for [`select`], [`poll`], [`epoll_wait`], and so on.
pub const SO_SNDTIMEO: c_int = 21;
