use crate::{
    sys::socket::{AF_INET, AF_INET6},
    SocketAddress,
};
use std::ffi::c_int;

impl SocketAddress {
    /// Gets the family that this socket address is for
    pub fn family(&self) -> c_int {
        match self {
            SocketAddress::V4(_) => AF_INET,
            SocketAddress::V6(_) => AF_INET6,
        }
    }
}
