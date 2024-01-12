use raw::sys::socket;

// rustdoc imports
#[allow(unused_imports)]
use crate::Socket;

/// Socket types used by [`Socket::new`]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketType {
    /// Sequenced, reliable, connection-based byte stream
    Stream = socket::SOCK_STREAM,

    /// Connectionless, unreliable datagrams of fixed maximum length
    Datagram = socket::SOCK_DGRAM,

    /// Reliably-devlivered messages
    RDM = socket::SOCK_RDM,

    /// Sequenced, reliable, connection-based, datagrams of fixed maximum length
    SeqPacket = socket::SOCK_SEQPACKET,

    /// Datagram Congestion Control Protocol
    DCCP = socket::SOCK_DCCP,

    /// Linux specific way of getting packets at the dev level. For writing rarp and other similar
    /// things on the user level.
    Packet = socket::SOCK_PACKET,
}
