use crate::sys::socket::sa_family_t;
use std::ffi::c_char;

/// A socket address
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct sockaddr {
    /// Address family
    pub family: sa_family_t,

    /// Socket address
    pub data: [c_char; 14],
}
