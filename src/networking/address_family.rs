use crate::raw;

/// Address families used by [`Socket::new`]
#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AddressFamily {
    /// Unspecified.
    Unspecified = raw::AF_UNSPEC,

    /// Local to host (pipes and file-domain).
    Local = raw::AF_LOCAL,

    /// IP protocol family.
    INet = raw::AF_INET,

    /// Amateur Radio AX.25.
    AX25 = raw::AF_AX25,

    /// Novell Internet Protocol.
    IPX = raw::AF_IPX,

    /// Appletalk DDP.
    Appletalk = raw::AF_APPLETALK,

    /// Amateur radio NetROM.
    NetROM = raw::AF_NETROM,

    /// Multiprotocol bridge.
    Bridge = raw::AF_BRIDGE,

    /// ATM PVCs.
    ATMPVC = raw::AF_ATMPVC,

    /// Reserved for X.25 project.
    X25 = raw::AF_X25,

    /// IP version 6.
    INet6 = raw::AF_INET6,

    /// Amateur Radio X.25 PLP.
    Rose = raw::AF_ROSE,

    /// Reserved for DECnet project.
    DECnet = raw::AF_DECNET,

    /// Reserved for 802.2LLC project.
    NetBEUI = raw::AF_NETBEUI,

    /// Security callback pseudo AF.
    Security = raw::AF_SECURITY,

    /// PF_KEY key management API.
    Key = raw::AF_KEY,

    /// Netlink API
    Netlink = raw::AF_NETLINK,

    /// Packet family.
    Packet = raw::AF_PACKET,

    /// Ash.
    Ash = raw::AF_ASH,

    /// Acorn Econet.
    Econet = raw::AF_ECONET,

    /// ATM SVCs.
    ATMSVC = raw::AF_ATMSVC,

    /// RDS sockets.
    RDS = raw::AF_RDS,

    /// Linux SNA Project
    SNA = raw::AF_SNA,

    /// IRDA sockets.
    IRDA = raw::AF_IRDA,

    /// PPPoX sockets.
    PPPoX = raw::AF_PPPOX,

    /// Wanpipe API sockets.
    Wanpipe = raw::AF_WANPIPE,

    /// Linux LLC.
    LLC = raw::AF_LLC,

    /// Native InfiniBand address.
    IB = raw::AF_IB,

    /// MPLS.
    MPLS = raw::AF_MPLS,

    /// Controller Area Network.
    CAN = raw::AF_CAN,

    /// TIPC sockets.
    TIPC = raw::AF_TIPC,

    /// Bluetooth sockets.
    Bluetooth = raw::AF_BLUETOOTH,

    /// IUCV sockets.
    IUCV = raw::AF_IUCV,

    /// RxRPC sockets.
    RxRPC = raw::AF_RXRPC,

    /// mISDN sockets.
    ISDN = raw::AF_ISDN,

    /// Phonet sockets.
    Phonet = raw::AF_PHONET,

    /// IEEE 802.15.4 sockets.
    IEEE802154 = raw::AF_IEEE802154,

    /// CAIF sockets.
    Caif = raw::AF_CAIF,

    /// Algorithm sockets.
    Alg = raw::AF_ALG,

    /// NFC sockets.
    NFC = raw::AF_NFC,

    /// vSockets.
    VSock = raw::AF_VSOCK,

    /// Kernel Connection Multiplexor.
    KCM = raw::AF_KCM,

    /// Qualcomm IPC Router.
    QIPCRouter = raw::AF_QIPCRTR,

    /// SMC sockets.
    SMC = raw::AF_SMC,

    /// XDP sockets.
    XDP = raw::AF_XDP,

    /// Management component transport protocol.
    MCTP = raw::AF_MCTP,
}
