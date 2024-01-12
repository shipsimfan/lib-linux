//! I/O event notification facility

mod functions;

pub use functions::{epoll_create, epoll_create1};
