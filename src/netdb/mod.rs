//! Definitions for network database operations

mod functions;
mod structures;

pub use functions::{freeaddrinfo, getaddrinfo};
pub use structures::addrinfo;
