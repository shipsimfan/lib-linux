use crate::netinet::r#in::{in6_addr, in_port_t};
use std::{ffi::c_ushort, net::SocketAddrV6};

// rustdoc imports
#[allow(unused_imports)]
use crate::sys::socket::AF_INET6;

/// Structure describing an IPv6 socket address
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct sockaddr_in6 {
    /// [`AF_INET6`]
    pub family: c_ushort,

    /// Transport layer port #
    pub port: in_port_t,

    /// IPv6 flow information
    pub flow_info: u32,

    /// IPv6 address
    pub addr: in6_addr,

    /// IPv6 scope-id
    pub scope_id: u32,
}

impl Default for sockaddr_in6 {
    fn default() -> Self {
        sockaddr_in6 {
            family: AF_INET6 as c_ushort,
            port: 0,
            flow_info: 0,
            addr: in6_addr::default(),
            scope_id: 0,
        }
    }
}

impl From<SocketAddrV6> for sockaddr_in6 {
    fn from(value: SocketAddrV6) -> Self {
        sockaddr_in6 {
            family: AF_INET6 as c_ushort,
            port: value.port(),
            flow_info: value.flowinfo(),
            addr: (*value.ip()).into(),
            scope_id: value.scope_id(),
        }
    }
}

impl Into<SocketAddrV6> for sockaddr_in6 {
    fn into(self) -> SocketAddrV6 {
        SocketAddrV6::new(self.addr.into(), self.port, self.flow_info, self.scope_id)
    }
}
