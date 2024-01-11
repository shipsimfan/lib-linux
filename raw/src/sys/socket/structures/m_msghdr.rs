use crate::sys::{socket::socklen_t, uio::iovec};
use core::ffi::{c_size_t, c_void};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::sys::socket::{recvmsg, sendmsg};

/// Structure describing messages sent by [`sendmsg`] and received by [`recvmsg`]
#[repr(C)]
#[allow(non_camel_case_types)]
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
