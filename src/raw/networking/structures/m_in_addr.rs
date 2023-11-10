use crate::raw::in_addr_t;

/// Internet address
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct in_addr {
    /// Address
    pub addr: in_addr_t,
}
