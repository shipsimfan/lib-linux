use crate::{
    c_size_t,
    sys::{socket::socklen_t, uio::iovec},
};
use core::ffi::c_void;
use std::{ffi::c_int, ptr::null_mut};

// rustdoc imports
#[allow(unused_imports)]
use crate::sys::socket::{recvmsg, sendmsg};

/// Structure describing messages sent by [`sendmsg`] and received by [`recvmsg`]
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct msghdr {
    /// Address to send to/receive from
    pub msg_name: *mut c_void,

    /// Length of address data
    pub msg_namelen: socklen_t,

    /// Vector of data to send/receive into
    pub msg_iov: *mut iovec,

    /// Number of elements in the vector
    pub msg_iovlen: c_size_t,

    /// Ancillary data
    pub msg_control: *mut c_void,

    /// Ancillary data buffer length
    pub msg_controllen: c_size_t,

    /// Flags on received message
    pub msg_flags: c_int,
}

impl Default for msghdr {
    fn default() -> Self {
        msghdr {
            msg_name: null_mut(),
            msg_namelen: 0,
            msg_iov: null_mut(),
            msg_iovlen: 0,
            msg_control: null_mut(),
            msg_controllen: 0,
            msg_flags: 0,
        }
    }
}
