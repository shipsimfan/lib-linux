use crate::{
    netinet::r#in::{in_addr, in_port_t},
    sys::socket::sockaddr,
};
use std::{ffi::c_ushort, mem::size_of};

// rustdoc imports
#[allow(unused_imports)]
use crate::sys::socket::AF_INET;

const PADDING_SIZE: usize =
    size_of::<sockaddr>() - size_of::<c_ushort>() - size_of::<in_port_t>() - size_of::<in_addr>();

/// Structure describing an Internet socket address
#[repr(C)]
#[allow(non_camel_case_types)]
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
