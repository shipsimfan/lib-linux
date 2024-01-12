use crate::{
    netinet::r#in::{in_addr, in_port_t},
    sys::socket::{sockaddr, AF_INET},
};
use std::{ffi::c_ushort, mem::size_of, net::SocketAddrV4};

const PADDING_SIZE: usize =
    size_of::<sockaddr>() - size_of::<c_ushort>() - size_of::<in_port_t>() - size_of::<in_addr>();

/// Structure describing an Internet socket address
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct sockaddr_in {
    /// [`AF_INET`]
    pub family: c_ushort,

    /// Port number
    pub port: in_port_t,

    /// Internet address
    pub addr: in_addr,

    /// Pad to size of [`sockaddr`]
    pub zero: [u8; PADDING_SIZE],
}

impl Default for sockaddr_in {
    fn default() -> Self {
        sockaddr_in {
            family: AF_INET as c_ushort,
            port: 0,
            addr: in_addr::default(),
            zero: [0; PADDING_SIZE],
        }
    }
}

impl From<SocketAddrV4> for sockaddr_in {
    fn from(value: SocketAddrV4) -> Self {
        sockaddr_in {
            family: AF_INET as c_ushort,
            port: value.port(),
            addr: (*value.ip()).into(),
            zero: [0; PADDING_SIZE],
        }
    }
}

impl Into<SocketAddrV4> for sockaddr_in {
    fn into(self) -> SocketAddrV4 {
        SocketAddrV4::new(self.addr.into(), self.port)
    }
}
