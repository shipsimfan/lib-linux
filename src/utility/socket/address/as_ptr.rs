use crate::{sys::socket::sockaddr, SocketAddress};

impl SocketAddress {
    /// Gets a pointer to the underlying socket address
    pub fn as_ptr(&self) -> *const sockaddr {
        match self {
            SocketAddress::V4(addr) => addr as *const _ as _,
            SocketAddress::V6(addr) => addr as *const _ as _,
        }
    }

    /// Gets a mutable pointer to the underlying socket address
    pub fn as_mut_ptr(&mut self) -> *mut sockaddr {
        match self {
            SocketAddress::V4(addr) => addr as *mut _ as _,
            SocketAddress::V6(addr) => addr as *mut _ as _,
        }
    }
}
