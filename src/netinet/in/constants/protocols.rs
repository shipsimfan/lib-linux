use std::ffi::c_int;

/// Dummy protocol for TCP.
pub const IPPROTO_IP: c_int = 0;

/// Internet Control Message Protocol.
pub const IPPROTO_ICMP: c_int = 1;

/// Internet Group Management Protocol.
pub const IPPROTO_IGMP: c_int = 2;

/// IPIP tunnels (older KA9Q tunnels use 94).
pub const IPPROTO_IPIP: c_int = 4;

/// Transmission Control Protocol.
pub const IPPROTO_TCP: c_int = 6;

/// Exterior Gateway Protocol.
pub const IPPROTO_EGP: c_int = 8;

/// PUP protocol.
pub const IPPROTO_PUP: c_int = 12;

/// User Datagram Protocol.
pub const IPPROTO_UDP: c_int = 17;

/// XNS IDP protocol.
pub const IPPROTO_IDP: c_int = 22;

/// SO Transport Protocol Class 4.
pub const IPPROTO_TP: c_int = 29;

/// Datagram Congestion Control Protocol.
pub const IPPROTO_DCCP: c_int = 33;

/// IPv6 header.
pub const IPPROTO_IPV6: c_int = 41;

/// Reservation Protocol.
pub const IPPROTO_RSVP: c_int = 46;

/// General Routing Encapsulation.
pub const IPPROTO_GRE: c_int = 47;

/// encapsulating security payload.
pub const IPPROTO_ESP: c_int = 50;

/// authentication header.
pub const IPPROTO_AH: c_int = 51;

/// Multicast Transport Protocol.
pub const IPPROTO_MTP: c_int = 92;

/// IP option pseudo header for BEET.
pub const IPPROTO_BEETPH: c_int = 94;

/// Encapsulation Header.
pub const IPPROTO_ENCAP: c_int = 98;

/// Protocol Independent Multicast.
pub const IPPROTO_PIM: c_int = 103;

/// Compression Header Protocol.
pub const IPPROTO_COMP: c_int = 108;

/// Layer 2 Tunnelling Protocol.
pub const IPPROTO_L2TP: c_int = 115;

/// Stream Control Transmission Protocol.
pub const IPPROTO_SCTP: c_int = 132;

/// UDP-Lite protocol.
pub const IPPROTO_UDPLITE: c_int = 136;

/// MPLS in IP.
pub const IPPROTO_MPLS: c_int = 137;

/// Ethernet-within-IPv6 Encapsulation.
pub const IPPROTO_ETHERNET: c_int = 143;

/// Raw IP packets.
pub const IPPROTO_RAW: c_int = 255;

/// Multipath TCP connection.
pub const IPPROTO_MPTCP: c_int = 262;
