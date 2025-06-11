use crate::sys::socket::AF_UNIX;
use std::ffi::c_ushort;

/// Structure describing the address of an [`AF_LOCAL`] (aka [`AF_UNIX`]) socket
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct sockaddr_un {
    /// [`AF_UNIX`]
    pub family: c_ushort,

    /// Path name
    pub sun_path: [u8; 108],
}

impl Default for sockaddr_un {
    fn default() -> Self {
        sockaddr_un {
            family: AF_UNIX as _,
            sun_path: [0; 108],
        }
    }
}
