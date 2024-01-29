//! Standard symbolic constants and types

mod functions;
mod types;

pub use functions::{close, geteuid, getuid, read, write};
pub use types::pid_t;
