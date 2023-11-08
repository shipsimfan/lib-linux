use crate::raw;

/// Socket types used by [`Socket::new`]
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SocketType {
    /// Sequenced, reliable, connection-based byte stream
    Stream = raw::SOCK_STREAM,

    /// Connectionless, unreliable datagrams of fixed maximum length
    Datagram = raw::SOCK_DGRAM,

    /// Raw protocol interface
    Raw = raw::SOCK_RAW,

    /// Reliably-devlivered messages
    RDM = raw::SOCK_RDM,

    /// Sequenced, reliable, connection-based, datagrams of fixed maximum length
    SeqPacket = raw::SOCK_SEQPACKET,

    /// Datagram Congestion Control Protocol
    DCCP = raw::SOCK_DCCP,

    /// Linux specific way of getting packets at the dev level. For writing rarp and other similar
    /// things on the user level.
    Packet = raw::SOCK_PACKET,
}
