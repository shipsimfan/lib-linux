use std::ffi::c_ushort;

// rustdoc imports
#[allow(unused_imports)]
use crate::sys::socket::AF_NETLINK;

/// Structure describing a Netlink socket address
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct sockaddr_nl {
    /// [`AF_NETLINK`]
    pub family: c_ushort,

    /// Zero
    pub pad: c_ushort,

    /// Port ID
    pub pid: u32,

    /// Multicast groups mask
    pub groups: u32,
}
