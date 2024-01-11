/// IPv6 address
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct in6_addr {
    /// Address
    pub addr: [u8; 16],
}
