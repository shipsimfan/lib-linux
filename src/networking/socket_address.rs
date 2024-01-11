use raw::{
    linux::netlink::sockaddr_nl,
    netinet::r#in::{sockaddr_in, sockaddr_in6},
    sys::socket::{sockaddr, socklen_t, AF_INET, AF_INET6, AF_NETLINK, AF_UNSPEC},
};

/// A socket address type
pub trait SocketAddress: Sized {
    /// The family used by the socket address
    const FAMILY: u16;

    /// The size of the socket address
    const SIZE: socklen_t;

    /// Returns the family of a given socket address
    fn family(&self) -> u16;

    /// Converts this object to a `*const sockaddr`
    unsafe fn as_sockaddr_ptr(&self) -> *const sockaddr {
        self as *const Self as *const sockaddr
    }

    /// Converts this object to a `*const sockaddr`
    unsafe fn as_mut_sockaddr_ptr(&mut self) -> *mut sockaddr {
        self as *mut Self as *mut sockaddr
    }
}

impl SocketAddress for sockaddr {
    const FAMILY: u16 = AF_UNSPEC as u16;
    const SIZE: socklen_t = std::mem::size_of::<Self>() as u32;

    fn family(&self) -> u16 {
        self.family
    }
}

impl SocketAddress for sockaddr_in {
    const FAMILY: u16 = AF_INET as u16;
    const SIZE: socklen_t = std::mem::size_of::<Self>() as u32;

    fn family(&self) -> u16 {
        self.family
    }
}

impl SocketAddress for sockaddr_in6 {
    const FAMILY: u16 = AF_INET6 as u16;
    const SIZE: socklen_t = std::mem::size_of::<Self>() as u32;

    fn family(&self) -> u16 {
        self.family
    }
}

impl SocketAddress for sockaddr_nl {
    const FAMILY: u16 = AF_NETLINK as u16;
    const SIZE: socklen_t = std::mem::size_of::<Self>() as u32;

    fn family(&self) -> u16 {
        self.family
    }
}
