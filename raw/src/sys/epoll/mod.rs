//! I/O event notification facility

mod functions;
mod structures;

pub use functions::{epoll_create, epoll_create1, epoll_ctl, epoll_wait};
pub use structures::{epoll_data_t, epoll_event};
