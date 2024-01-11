use crate::netinet::r#in::{in6_addr, in_port_t};
use std::ffi::c_ushort;

// rustdoc imports
#[allow(unused_imports)]
use crate::sys::socket::AF_INET6;

/// Structure describing an IPv6 socket address
#[repr(C)]
#[allow(non_camel_case_types)]
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
