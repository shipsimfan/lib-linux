use std::ffi::{c_uchar, c_ushort};

/// Socket address for any protocol
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct sockaddr {
    /// The address family this socket address is
    pub family: c_ushort,

    /// The data describing the socket address
    pub data: [c_uchar; 14],
}
