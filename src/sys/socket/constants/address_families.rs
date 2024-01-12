use super::protocol_families::*;
use std::ffi::c_int;

/// Unspecified.
pub const AF_UNSPEC: c_int = PF_UNSPEC;

/// Local to host (pipes and file-domain).
pub const AF_LOCAL: c_int = PF_LOCAL;

/// POSIX name for PF_LOCAL.
pub const AF_UNIX: c_int = PF_UNIX;

/// Another non-standard name for PF_LOCAL.
pub const AF_FILE: c_int = PF_FILE;

/// IP protocol family.
pub const AF_INET: c_int = PF_INET;

/// Amateur Radio AX.25.
pub const AF_AX25: c_int = PF_AX25;

/// Novell Internet Protocol.
pub const AF_IPX: c_int = PF_IPX;

/// Appletalk DDP.
pub const AF_APPLETALK: c_int = PF_APPLETALK;

/// Amateur radio NetROM.
pub const AF_NETROM: c_int = PF_NETROM;

/// Multiprotocol bridge.
pub const AF_BRIDGE: c_int = PF_BRIDGE;

/// ATM PVCs.
pub const AF_ATMPVC: c_int = PF_ATMPVC;

/// Reserved for X.25 project.
pub const AF_X25: c_int = PF_X25;

/// IP version 6.
pub const AF_INET6: c_int = PF_INET6;

/// Amateur Radio X.25 PLP.
pub const AF_ROSE: c_int = PF_ROSE;

/// Reserved for DECnet project.
pub const AF_DECNET: c_int = PF_DECNET;

/// Reserved for 802.2LLC project.
pub const AF_NETBEUI: c_int = PF_NETBEUI;

/// Security callback pseudo AF.
pub const AF_SECURITY: c_int = PF_SECURITY;

/// PF_KEY key management API.
pub const AF_KEY: c_int = PF_KEY;

/// Netlink API
pub const AF_NETLINK: c_int = PF_NETLINK;

/// Alias to emulate 4.4BSD.
pub const AF_ROUTE: c_int = PF_ROUTE;

/// Packet family.
pub const AF_PACKET: c_int = PF_PACKET;

/// Ash.
pub const AF_ASH: c_int = PF_ASH;

/// Acorn Econet.
pub const AF_ECONET: c_int = PF_ECONET;

/// ATM SVCs.
pub const AF_ATMSVC: c_int = PF_ATMSVC;

/// RDS sockets.
pub const AF_RDS: c_int = PF_RDS;

/// Linux SNA Project
pub const AF_SNA: c_int = PF_SNA;

/// IRDA sockets.
pub const AF_IRDA: c_int = PF_IRDA;

/// PPPoX sockets.
pub const AF_PPPOX: c_int = PF_PPPOX;

/// Wanpipe API sockets.
pub const AF_WANPIPE: c_int = PF_WANPIPE;

/// Linux LLC.
pub const AF_LLC: c_int = PF_LLC;

/// Native InfiniBand address.
pub const AF_IB: c_int = PF_IB;

/// MPLS.
pub const AF_MPLS: c_int = PF_MPLS;

/// Controller Area Network.
pub const AF_CAN: c_int = PF_CAN;

/// TIPC sockets.
pub const AF_TIPC: c_int = PF_TIPC;

/// Bluetooth sockets.
pub const AF_BLUETOOTH: c_int = PF_BLUETOOTH;

/// IUCV sockets.
pub const AF_IUCV: c_int = PF_IUCV;

/// RxRPC sockets.
pub const AF_RXRPC: c_int = PF_RXRPC;

/// mISDN sockets.
pub const AF_ISDN: c_int = PF_ISDN;

/// Phonet sockets.
pub const AF_PHONET: c_int = PF_PHONET;

/// IEEE 802.15.4 sockets.
pub const AF_IEEE802154: c_int = PF_IEEE802154;

/// CAIF sockets.
pub const AF_CAIF: c_int = PF_CAIF;

/// Algorithm sockets.
pub const AF_ALG: c_int = PF_ALG;

/// NFC sockets.
pub const AF_NFC: c_int = PF_NFC;

/// vSockets.
pub const AF_VSOCK: c_int = PF_VSOCK;

/// Kernel Connection Multiplexor.
pub const AF_KCM: c_int = PF_KCM;

/// Qualcomm IPC Router.
pub const AF_QIPCRTR: c_int = PF_QIPCRTR;

/// SMC sockets.
pub const AF_SMC: c_int = PF_SMC;

/// XDP sockets.
pub const AF_XDP: c_int = PF_XDP;

/// Management component transport protocol.
pub const AF_MCTP: c_int = PF_MCTP;

/// For now..
pub const AF_MAX: c_int = PF_MAX;
