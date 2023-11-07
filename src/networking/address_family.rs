use crate::raw::{
    AF_ALG, AF_APPLETALK, AF_ASH, AF_ATMPVC, AF_ATMSVC, AF_AX25, AF_BLUETOOTH, AF_BRIDGE, AF_CAIF,
    AF_CAN, AF_DECNET, AF_ECONET, AF_IB, AF_IEEE802154, AF_INET, AF_INET6, AF_IPX, AF_IRDA,
    AF_ISDN, AF_IUCV, AF_KCM, AF_KEY, AF_LLC, AF_LOCAL, AF_MCTP, AF_MPLS, AF_NETBEUI, AF_NETLINK,
    AF_NETROM, AF_NFC, AF_PACKET, AF_PHONET, AF_PPPOX, AF_QIPCRTR, AF_RDS, AF_ROSE, AF_RXRPC,
    AF_SECURITY, AF_SMC, AF_SNA, AF_TIPC, AF_UNSPEC, AF_VSOCK, AF_WANPIPE, AF_X25, AF_XDP,
};

/// Address families used by [`Socket::new`]
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AddressFamily {
    /// Unspecified.
    Unspecified = AF_UNSPEC,

    /// Local to host (pipes and file-domain).
    Local = AF_LOCAL,

    /// IP protocol family.
    INet = AF_INET,

    /// Amateur Radio AX.25.
    AX25 = AF_AX25,

    /// Novell Internet Protocol.
    IPX = AF_IPX,

    /// Appletalk DDP.
    Appletalk = AF_APPLETALK,

    /// Amateur radio NetROM.
    NetROM = AF_NETROM,

    /// Multiprotocol bridge.
    Bridge = AF_BRIDGE,

    /// ATM PVCs.
    ATMPVC = AF_ATMPVC,

    /// Reserved for X.25 project.
    X25 = AF_X25,

    /// IP version 6.
    INet6 = AF_INET6,

    /// Amateur Radio X.25 PLP.
    Rose = AF_ROSE,

    /// Reserved for DECnet project.
    DECnet = AF_DECNET,

    /// Reserved for 802.2LLC project.
    NetBEUI = AF_NETBEUI,

    /// Security callback pseudo AF.
    Security = AF_SECURITY,

    /// PF_KEY key management API.
    Key = AF_KEY,

    /// Netlink API
    Netlink = AF_NETLINK,

    /// Packet family.
    Packet = AF_PACKET,

    /// Ash.
    Ash = AF_ASH,

    /// Acorn Econet.
    Econet = AF_ECONET,

    /// ATM SVCs.
    ATMSVC = AF_ATMSVC,

    /// RDS sockets.
    RDS = AF_RDS,

    /// Linux SNA Project
    SNA = AF_SNA,

    /// IRDA sockets.
    IRDA = AF_IRDA,

    /// PPPoX sockets.
    PPPoX = AF_PPPOX,

    /// Wanpipe API sockets.
    Wanpipe = AF_WANPIPE,

    /// Linux LLC.
    LLC = AF_LLC,

    /// Native InfiniBand address.
    IB = AF_IB,

    /// MPLS.
    MPLS = AF_MPLS,

    /// Controller Area Network.
    CAN = AF_CAN,

    /// TIPC sockets.
    TIPC = AF_TIPC,

    /// Bluetooth sockets.
    Bluetooth = AF_BLUETOOTH,

    /// IUCV sockets.
    IUCV = AF_IUCV,

    /// RxRPC sockets.
    RxRPC = AF_RXRPC,

    /// mISDN sockets.
    ISDN = AF_ISDN,

    /// Phonet sockets.
    Phonet = AF_PHONET,

    /// IEEE 802.15.4 sockets.
    IEEE802154 = AF_IEEE802154,

    /// CAIF sockets.
    Caif = AF_CAIF,

    /// Algorithm sockets.
    Alg = AF_ALG,

    /// NFC sockets.
    NFC = AF_NFC,

    /// vSockets.
    VSock = AF_VSOCK,

    /// Kernel Connection Multiplexor.
    KCM = AF_KCM,

    /// Qualcomm IPC Router.
    QIPCRouter = AF_QIPCRTR,

    /// SMC sockets.
    SMC = AF_SMC,

    /// XDP sockets.
    XDP = AF_XDP,

    /// Management component transport protocol.
    MCTP = AF_MCTP,
}
