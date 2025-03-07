use std::ffi::c_int;

mod address;

mod deref;
mod drop;
mod get;
mod new;

pub use address::SocketAddress;

/// A wrapper around a linux socket
pub struct Socket {
    /// The handle to the socket
    handle: c_int,
}
