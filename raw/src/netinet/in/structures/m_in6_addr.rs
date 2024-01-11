use std::net::Ipv6Addr;

/// IPv6 address
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct in6_addr {
    /// Address
    pub addr: [u8; 16],
}

impl From<Ipv6Addr> for in6_addr {
    fn from(value: Ipv6Addr) -> Self {
        in6_addr {
            addr: value.octets(),
        }
    }
}

impl Into<Ipv6Addr> for in6_addr {
    fn into(self) -> Ipv6Addr {
        Ipv6Addr::from(self.addr)
    }
}
