use std::ffi::c_int;

/// Sequenced, reliable, connection-based byte stream
pub const SOCK_STREAM: c_int = 1;

/// Connectionless, unreliable datagrams of fixed maximum length
pub const SOCK_DGRAM: c_int = 2;

/// Raw protocol interface
pub const SOCK_RAW: c_int = 3;

/// Reliably-devlivered messages
pub const SOCK_RDM: c_int = 4;

/// Sequenced, reliable, connection-based, datagrams of fixed maximum length
pub const SOCK_SEQPACKET: c_int = 5;

/// Datagram Congestion Control Protocol
pub const SOCK_DCCP: c_int = 6;

/// Linux specific way of getting packets at the dev level. For writing rarp and other similar
/// things on the user level.
pub const SOCK_PACKET: c_int = 10;
