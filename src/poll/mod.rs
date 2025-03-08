//! Definitions for the [`poll`] function

mod constants;
mod functions;
mod structures;
mod types;

pub use constants::*;
pub use functions::poll;
pub use structures::pollfd;
pub use types::nfds_t;
