use std::ffi::c_int;

mod address;

mod accept;
mod bind;
mod connect;
mod deref;
mod drop;
mod get;
mod listen;
mod new;
mod recv;
mod send;

pub use address::SocketAddress;

/// A wrapper around a linux socket
pub struct Socket {
    /// The handle to the socket
    handle: c_int,
}
