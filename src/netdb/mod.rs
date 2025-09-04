//! Definitions for network database operations

mod constants;
mod functions;
mod structures;

pub use constants::*;
pub use functions::{freeaddrinfo, gai_strerror, getaddrinfo};
pub use structures::addrinfo;
