/// A type representing the length of a socket address
#[allow(non_camel_case_types)]
pub type socklen_t = u32;

/// Type to represent a port
#[allow(non_camel_case_types)]
pub type in_port_t = u16;

/// Internet address
#[allow(non_camel_case_types)]
pub type in_addr_t = u32;

/// IPv6 address
#[allow(non_camel_case_types)]
pub type in6_addr = [u8; 16];
