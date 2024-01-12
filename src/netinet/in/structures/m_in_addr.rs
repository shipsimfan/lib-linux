use crate::netinet::r#in::in_addr_t;
use std::net::Ipv4Addr;

/// Internet address
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct in_addr {
    /// Address
    pub addr: in_addr_t,
}

impl Default for in_addr {
    fn default() -> Self {
        in_addr { addr: 0 }
    }
}

impl From<Ipv4Addr> for in_addr {
    fn from(value: Ipv4Addr) -> Self {
        in_addr { addr: value.into() }
    }
}

impl Into<Ipv4Addr> for in_addr {
    fn into(self) -> Ipv4Addr {
        Ipv4Addr::from(self.addr)
    }
}
