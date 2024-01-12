use raw::sys::socket;

// rustdoc imports
#[allow(unused_imports)]
use crate::Socket;

/// Address families used by [`Socket::new`]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressFamily {
    /// Unspecified.
    Unspecified = socket::AF_UNSPEC,

    /// Local to host (pipes and file-domain).
    Local = socket::AF_LOCAL,

    /// IP protocol family.
    INet = socket::AF_INET,

    /// Amateur Radio AX.25.
    AX25 = socket::AF_AX25,

    /// Novell Internet Protocol.
    IPX = socket::AF_IPX,

    /// Appletalk DDP.
    Appletalk = socket::AF_APPLETALK,

    /// Amateur radio NetROM.
    NetROM = socket::AF_NETROM,

    /// Multiprotocol bridge.
    Bridge = socket::AF_BRIDGE,

    /// ATM PVCs.
    ATMPVC = socket::AF_ATMPVC,

    /// Reserved for X.25 project.
    X25 = socket::AF_X25,

    /// IP version 6.
    INet6 = socket::AF_INET6,

    /// Amateur Radio X.25 PLP.
    Rose = socket::AF_ROSE,

    /// Reserved for DECnet project.
    DECnet = socket::AF_DECNET,

    /// Reserved for 802.2LLC project.
    NetBEUI = socket::AF_NETBEUI,

    /// Security callback pseudo AF.
    Security = socket::AF_SECURITY,

    /// PF_KEY key management API.
    Key = socket::AF_KEY,

    /// Netlink API
    Netlink = socket::AF_NETLINK,

    /// Packet family.
    Packet = socket::AF_PACKET,

    /// Ash.
    Ash = socket::AF_ASH,

    /// Acorn Econet.
    Econet = socket::AF_ECONET,

    /// ATM SVCs.
    ATMSVC = socket::AF_ATMSVC,

    /// RDS sockets.
    RDS = socket::AF_RDS,

    /// Linux SNA Project
    SNA = socket::AF_SNA,

    /// IRDA sockets.
    IRDA = socket::AF_IRDA,

    /// PPPoX sockets.
    PPPoX = socket::AF_PPPOX,

    /// Wanpipe API sockets.
    Wanpipe = socket::AF_WANPIPE,

    /// Linux LLC.
    LLC = socket::AF_LLC,

    /// Native InfiniBand address.
    IB = socket::AF_IB,

    /// MPLS.
    MPLS = socket::AF_MPLS,

    /// Controller Area Network.
    CAN = socket::AF_CAN,

    /// TIPC sockets.
    TIPC = socket::AF_TIPC,

    /// Bluetooth sockets.
    Bluetooth = socket::AF_BLUETOOTH,

    /// IUCV sockets.
    IUCV = socket::AF_IUCV,

    /// RxRPC sockets.
    RxRPC = socket::AF_RXRPC,

    /// mISDN sockets.
    ISDN = socket::AF_ISDN,

    /// Phonet sockets.
    Phonet = socket::AF_PHONET,

    /// IEEE 802.15.4 sockets.
    IEEE802154 = socket::AF_IEEE802154,

    /// CAIF sockets.
    Caif = socket::AF_CAIF,

    /// Algorithm sockets.
    Alg = socket::AF_ALG,

    /// NFC sockets.
    NFC = socket::AF_NFC,

    /// vSockets.
    VSock = socket::AF_VSOCK,

    /// Kernel Connection Multiplexor.
    KCM = socket::AF_KCM,

    /// Qualcomm IPC Router.
    QIPCRouter = socket::AF_QIPCRTR,

    /// SMC sockets.
    SMC = socket::AF_SMC,

    /// XDP sockets.
    XDP = socket::AF_XDP,

    /// Management component transport protocol.
    MCTP = socket::AF_MCTP,
}
