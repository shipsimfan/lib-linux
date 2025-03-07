use crate::{
    netinet::r#in::{sockaddr_in, sockaddr_in6},
    SocketAddress,
};

impl SocketAddress {
    /// Gets the length of the underlying socket address
    pub fn len(&self) -> usize {
        match self {
            SocketAddress::V4(_) => std::mem::size_of::<sockaddr_in>(),
            SocketAddress::V6(_) => std::mem::size_of::<sockaddr_in6>(),
        }
    }
}
