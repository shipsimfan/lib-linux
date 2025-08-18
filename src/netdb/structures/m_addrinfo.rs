use crate::sys::socket::{sockaddr, socklen_t};
use std::{
    ffi::{c_char, c_int},
    ptr::null_mut,
};

/// Structure to contain information about address of a service provider
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct addrinfo {
    /// Input flags
    pub flags: c_int,

    /// Protocol family for socket
    pub family: c_int,

    /// Socket type
    pub socktype: c_int,

    /// Protocol for socket
    pub protocol: c_int,

    /// Length of socket address
    pub addrlen: socklen_t,

    /// Socket address for socket
    pub addr: *mut sockaddr,

    /// Canonical name for service location
    pub canonname: *mut c_char,

    /// Poitner to next in list
    pub next: *mut addrinfo,
}

impl Default for addrinfo {
    fn default() -> Self {
        addrinfo {
            flags: 0,
            family: 0,
            socktype: 0,
            protocol: 0,
            addrlen: 0,
            addr: null_mut(),
            canonname: null_mut(),
            next: null_mut(),
        }
    }
}
