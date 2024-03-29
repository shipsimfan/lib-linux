//! Main sockets header
mod constants;
mod functions;
mod structures;
mod types;

pub use constants::*;
pub use functions::{
    accept, bind, connect, getpeername, getsockname, getsockopt, listen, recv, recvfrom, recvmsg,
    send, sendmsg, sendto, setsockopt, socket,
};
pub use structures::{msghdr, sockaddr};
pub use types::{sa_family_t, socklen_t};
