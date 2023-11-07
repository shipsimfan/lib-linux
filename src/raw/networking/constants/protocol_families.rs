use std::ffi::c_int;

/// Unspecified.
pub const PF_UNSPEC: c_int = 0;

/// Local to host (pipes and file-domain).
pub const PF_LOCAL: c_int = 1;

/// POSIX name for PF_LOCAL.
pub const PF_UNIX: c_int = PF_LOCAL;

/// Another non-standard name for PF_LOCAL.
pub const PF_FILE: c_int = PF_LOCAL;

/// IP protocol family.
pub const PF_INET: c_int = 2;

/// Amateur Radio AX.25.
pub const PF_AX25: c_int = 3;

/// Novell Internet Protocol.
pub const PF_IPX: c_int = 4;

/// Appletalk DDP.
pub const PF_APPLETALK: c_int = 5;

/// Amateur radio NetROM.
pub const PF_NETROM: c_int = 6;

/// Multiprotocol bridge.
pub const PF_BRIDGE: c_int = 7;

/// ATM PVCs.
pub const PF_ATMPVC: c_int = 8;

/// Reserved for X.25 project.
pub const PF_X25: c_int = 9;

/// IP version 6.
pub const PF_INET6: c_int = 10;

/// Amateur Radio X.25 PLP.
pub const PF_ROSE: c_int = 11;

/// Reserved for DECnet project.
pub const PF_DECNET: c_int = 12;

/// Reserved for 802.2LLC project.
pub const PF_NETBEUI: c_int = 13;

/// Security callback pseudo AF.
pub const PF_SECURITY: c_int = 14;

/// PF_KEY key management API.
pub const PF_KEY: c_int = 15;

/// Netlink API
pub const PF_NETLINK: c_int = 16;

/// Alias to emulate 4.4BSD.
pub const PF_ROUTE: c_int = PF_NETLINK;

/// Packet family.
pub const PF_PACKET: c_int = 17;

/// Ash.
pub const PF_ASH: c_int = 18;

/// Acorn Econet.
pub const PF_ECONET: c_int = 19;

/// ATM SVCs.
pub const PF_ATMSVC: c_int = 20;

/// RDS sockets.
pub const PF_RDS: c_int = 21;

/// Linux SNA Project
pub const PF_SNA: c_int = 22;

/// IRDA sockets.
pub const PF_IRDA: c_int = 23;

/// PPPoX sockets.
pub const PF_PPPOX: c_int = 24;

/// Wanpipe API sockets.
pub const PF_WANPIPE: c_int = 25;

/// Linux LLC.
pub const PF_LLC: c_int = 26;

/// Native InfiniBand address.
pub const PF_IB: c_int = 27;

/// MPLS.
pub const PF_MPLS: c_int = 28;

/// Controller Area Network.
pub const PF_CAN: c_int = 29;

/// TIPC sockets.
pub const PF_TIPC: c_int = 30;

/// Bluetooth sockets.
pub const PF_BLUETOOTH: c_int = 31;

/// IUCV sockets.
pub const PF_IUCV: c_int = 32;

/// RxRPC sockets.
pub const PF_RXRPC: c_int = 33;

/// mISDN sockets.
pub const PF_ISDN: c_int = 34;

/// Phonet sockets.
pub const PF_PHONET: c_int = 35;

/// IEEE 802.15.4 sockets.
pub const PF_IEEE802154: c_int = 36;

/// CAIF sockets.
pub const PF_CAIF: c_int = 37;

/// Algorithm sockets.
pub const PF_ALG: c_int = 38;

/// NFC sockets.
pub const PF_NFC: c_int = 39;

/// vSockets.
pub const PF_VSOCK: c_int = 40;

/// Kernel Connection Multiplexor.
pub const PF_KCM: c_int = 41;

/// Qualcomm IPC Router.
pub const PF_QIPCRTR: c_int = 42;

/// SMC sockets.
pub const PF_SMC: c_int = 43;

/// XDP sockets.
pub const PF_XDP: c_int = 44;

/// Management component transport protocol.
pub const PF_MCTP: c_int = 45;

/// For now..
pub const PF_MAX: c_int = 46;
