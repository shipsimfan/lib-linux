/// IPv6 address
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct in6_addr {
    /// Address
    pub addr: [u8; 16],
}
