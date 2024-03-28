use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{
        errno, EACCES, EAFNOSUPPORT, EINVAL, EMFILE, ENFILE, ENOBUFS, ENOMEM, EPROTONOSUPPORT,
    },
    signal::{SIGIO, SIGPIPE, SIGURG},
    sys::socket::{
        connect, getsockopt, recv, recvfrom, send, sendto, setsockopt, AF_ALG, AF_APPLETALK,
        AF_ASH, AF_ATMPVC, AF_ATMSVC, AF_AX25, AF_BLUETOOTH, AF_BRIDGE, AF_CAIF, AF_CAN, AF_DECNET,
        AF_ECONET, AF_FILE, AF_IB, AF_IEEE802154, AF_INET, AF_INET6, AF_IPX, AF_IRDA, AF_ISDN,
        AF_IUCV, AF_KCM, AF_KEY, AF_LLC, AF_LOCAL, AF_MCTP, AF_MPLS, AF_NETBEUI, AF_NETLINK,
        AF_NETROM, AF_NFC, AF_PACKET, AF_PHONET, AF_PPPOX, AF_QIPCRTR, AF_RDS, AF_ROSE, AF_ROUTE,
        AF_RXRPC, AF_SECURITY, AF_SMC, AF_SNA, AF_TIPC, AF_UNIX, AF_UNSPEC, AF_VSOCK, AF_WANPIPE,
        AF_X25, AF_XDP, SOCK_DCCP, SOCK_DGRAM, SOCK_PACKET, SOCK_RAW, SOCK_RDM, SOCK_SEQPACKET,
        SOCK_STREAM, SO_KEEPALIVE,
    },
    unistd::close,
};

extern "C" {
    /// [`socket`] creates an endpoint for communication and returns a file descriptor that refers
    /// to that endpoint. The file descriptor returned by a successful call will be the
    /// lowest-numbered file descriptor not currently open for the process.
    ///
    /// The `domain` argument specifies a communication domain; this selects the protocol family
    /// which will be use for communication. These families are defined in `<sys/socket.h>`. The
    /// formats currently understood by the linux kernel include:
    ///  * [`AF_UNSPEC`] - Unspecified.
    ///  * [`AF_LOCAL`] - Local to host (pipes and file-domain).
    ///  * [`AF_UNIX`] - POSIX name for PF_LOCAL.
    ///  * [`AF_FILE`] - Another non-standard name for PF_LOCAL.
    ///  * [`AF_INET`] - IP protocol family.
    ///  * [`AF_AX25`] - Amateur Radio AX.25.
    ///  * [`AF_IPX`] - Novell Internet Protocol.
    ///  * [`AF_APPLETALK`] - Appletalk DDP.
    ///  * [`AF_NETROM`] - Amateur radio NetROM.
    ///  * [`AF_BRIDGE`] - Multiprotocol bridge.
    ///  * [`AF_ATMPVC`] - ATM PVCs.
    ///  * [`AF_X25`] - Reserved for X.25 project.
    ///  * [`AF_INET6`] - IP version 6.
    ///  * [`AF_ROSE`] - Amateur Radio X.25 PLP.
    ///  * [`AF_DECNET`] - Reserved for DECnet project.
    ///  * [`AF_NETBEUI`] - Reserved for 802.2LLC project.
    ///  * [`AF_SECURITY`] - Security callback pseudo AF.
    ///  * [`AF_KEY`] - PF_KEY key management API.
    ///  * [`AF_NETLINK`] - Netlink API
    ///  * [`AF_ROUTE`] - Alias to emulate 4.4BSD.
    ///  * [`AF_PACKET`] - Packet family.
    ///  * [`AF_ASH`] - Ash.
    ///  * [`AF_ECONET`] - Acorn Econet.
    ///  * [`AF_ATMSVC`] - ATM SVCs.
    ///  * [`AF_RDS`] - RDS sockets.
    ///  * [`AF_SNA`] - Linux SNA Project
    ///  * [`AF_IRDA`] - IRDA sockets.
    ///  * [`AF_PPPOX`] - PPPoX sockets.
    ///  * [`AF_WANPIPE`] - Wanpipe API sockets.
    ///  * [`AF_LLC`] - Linux LLC.
    ///  * [`AF_IB`] - Native InfiniBand address.
    ///  * [`AF_MPLS`] - MPLS.
    ///  * [`AF_CAN`] - Controller Area Network.
    ///  * [`AF_TIPC`] - TIPC sockets.
    ///  * [`AF_BLUETOOTH`] - Bluetooth sockets.
    ///  * [`AF_IUCV`] - IUCV sockets.
    ///  * [`AF_RXRPC`] - RxRPC sockets.
    ///  * [`AF_ISDN`] - mISDN sockets.
    ///  * [`AF_PHONET`] - Phonet sockets.
    ///  * [`AF_IEEE802154`] - IEEE 802.15.4 sockets.
    ///  * [`AF_CAIF`] - CAIF sockets.
    ///  * [`AF_ALG`] - Algorithm sockets.
    ///  * [`AF_NFC`] - NFC sockets.
    ///  * [`AF_VSOCK`] - vSockets.
    ///  * [`AF_KCM`] - Kernel Connection Multiplexor.
    ///  * [`AF_QIPCRTR`] - Qualcomm IPC Router.
    ///  * [`AF_SMC`] - SMC sockets.
    ///  * [`AF_XDP`] - XDP sockets.
    ///  * [`AF_MCTP`] - Management component transport protocol.
    ///
    /// The socket as the indicated `r#type`, which specifies the communication semantics.
    /// Currently defined types are:
    ///  * [`SOCK_STREAM`] - Sequenced, reliable, connection-based byte stream
    ///  * [`SOCK_DGRAM`] - Connectionless, unreliable datagrams of fixed maximum length
    ///  * [`SOCK_RAW`] - Raw protocol interface
    ///  * [`SOCK_RDM`] - Reliably-devlivered messages
    ///  * [`SOCK_SEQPACKET`] - Sequenced, reliable, connection-based, datagrams of fixed maximum length
    ///  * [`SOCK_DCCP`] - Datagram Congestion Control Protocol
    ///  * [`SOCK_PACKET`] -  Obsolete and should not be used in new programs; see [`packet`].
    ///
    /// Some socket types may not be implemented by all protocol families.
    ///
    /// Since Linux 2.6.27, the type argument serves a second purpose: in addition to specifying
    /// a socket type, it may include the bitwise OR of any of the following values, to modify the
    /// behavior of [`socket`]:
    ///  * [`SOCK_NONBLOCK`]
    ///  * [`SOCK_CLOEXEC`]
    ///
    /// The `protocol` specifies a particular protocol to be used with the socket. Normally only a
    /// single protocol exists to support a particular socket type within a given protocol family,
    /// in which case `protocol` can be specified as 0. However, it is possible that many protocols
    /// may exist, in which case a particular protocol must be specified in this manner. The
    /// protocol number to use is specific to the “communication domain” in which communication is
    /// to take place; see `man 5 protocols`. See [`getprotoent`] on how to map protocol name
    /// strings to protocol numbers.
    ///
    /// Sockets of type [`SOCK_STREAM`] are full-duplex byte streams. They do not preserve record
    /// boundaries. A stream socket must be in a connected state before any data may be sent or
    /// received on it. A connection to another socket is created with a [`connect`] call. Once
    /// connected, data may be transferred using [`read`] and [`write()`] calls or some variant of
    /// the [`send`] and [`recv`] calls. When a session has been completed a [`close`] may be
    /// performed. Out-of-band data may also be transmitted as described in [`send`] and received
    /// as described in [`recv`].
    ///
    /// The communications protocols which implement a [`SOCK_STREAM`] ensure that data is not lost
    /// or duplicated. If a piece of data for which the peer protocol has buffer space cannot be
    /// successfully transmitted within a reasonable length of time, then the connection is
    /// considered to be dead. When [`SO_KEEPALIVE`] is enabled on the socket the protocol checks
    /// in a protocol-specific manner if the other end is still alive. A [`SIGPIPE`] signal is
    /// raised if a process sends or receives on a broken stream; this causes naive processes,
    /// which do not handle the signal, to exit. [`SOCK_SEQPACKET`] sockets employ the same system
    /// calls as [`SOCK_STREAM`] sockets. The only difference is that [`read`] calls will return
    /// only the amount of data requested, and any data remaining in the arriving packet will be
    /// discarded. Also all message boundaries in incoming datagrams are preserved.
    ///
    /// [`SOCK_DGRAM`] and [`SOCK_RAW`] sockets allow sending of datagrams to correspondents named
    /// in [`sendto`] calls. Datagrams are generally received with [`recvfrom`], which returns the
    /// next datagram along with the address of its sender.
    ///
    /// [`SOCK_PACKET`] is an obsolete socket type to receive raw packets directly from the device
    /// driver. Use [`packet`] instead.
    ///
    /// An [`fcntl`] [`F_SETOWN`] operation can be used to specify a process or process group to
    /// receive a [`SIGURG`] signal when the out-of-band data arrives or [`SIGPIPE`] signal when a
    /// [`SOCK_STREAM`] connection breaks unexpectedly. This operation may also be used to set the
    /// process or process group that receives the I/O and asynchronous notification of I/O events
    /// via [`SIGIO`]. Using [`F_SETOWN`] is equivalent to an [`ioctl`] call with the [`FIOSETOWN`]
    /// or [`SIOCSPGRP`] argument.
    ///
    /// When the network signals an error condition to the protocol module (e.g., using an ICMP
    /// message for IP) the pending error flag is set for the socket. The next operation on this
    /// socket will return the error code of the pending error. For some protocols it is possible
    /// to enable a per-socket error queue to retrieve detailed information about the error; see
    /// [`IP_RECVERR`] in [`ip`].
    ///
    /// The operation of sockets is controlled by socket level options. These options are defined
    /// in <sys/socket.h>. The functions [`setsockopt`] and [`getsockopt`] are used to set and get
    /// options.
    ///
    /// # Return Value
    /// On success, a file descriptor for the new socket is returned. On error, -1 is returned and
    /// [`errno`] is set to indicate the error.
    ///
    /// # Errors
    ///  * [`EACCES`] - Permission to create a socket of the specified type and/or protocol is
    ///                 denied.
    ///  * [`EAFNOSUPPORT`] - The implementation does not support the specified address family.
    ///  * [`EINVAL`] - Unknown protocol, or protocol family not available.
    ///  * [`EINVAL`] - Invalid flags in type.
    ///  * [`EMFILE`] - The per-process limit on the number of open file descriptors has been
    ///                 reached.
    ///  * [`ENFILE`] - The system-wide limit on the total number of open fileshas been reached.
    ///  * [`ENOBUFS`] or [`ENOMEM`] - Insufficient memory is available. The socket cannot be
    ///                                created until sufficient resources are freed.
    ///  * [`EPROTONOSUPPORT`] - The protocol type or the specified protocol is not supported
    ///                          within this domain.
    ///
    /// Other errors may be generated by the underlying protocol modules.
    pub fn socket(domain: c_int, r#type: c_int, protocol: c_int) -> c_int;
}
