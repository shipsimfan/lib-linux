use crate::raw::{
    SOCK_DCCP, SOCK_DGRAM, SOCK_PACKET, SOCK_RAW, SOCK_RDM, SOCK_SEQPACKET, SOCK_STREAM,
};

/// Socket types used by [`Socket::new`]
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SocketType {
    /// Sequenced, reliable, connection-based byte stream
    Stream = SOCK_STREAM,

    /// Connectionless, unreliable datagrams of fixed maximum length
    Datagram = SOCK_DGRAM,

    /// Raw protocol interface
    Raw = SOCK_RAW,

    /// Reliably-devlivered messages
    RDM = SOCK_RDM,

    /// Sequenced, reliable, connection-based, datagrams of fixed maximum length
    SeqPacket = SOCK_SEQPACKET,

    /// Datagram Congestion Control Protocol
    DCCP = SOCK_DCCP,

    /// Linux specific way of getting packets at the dev level. For writing rarp and other similar
    /// things on the user level.
    Packet = SOCK_PACKET,
}
