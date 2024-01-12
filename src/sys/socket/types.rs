use std::ffi::c_ushort;

/// A type representing the length of a socket address
#[allow(non_camel_case_types)]
pub type socklen_t = u32;

/// A type representing an address family
#[allow(non_camel_case_types)]
pub type sa_family_t = c_ushort;
