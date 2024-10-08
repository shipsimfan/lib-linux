//! Get and set file status

mod constants;
mod functions;
mod structures;
mod types;

pub use constants::*;
pub use functions::{
    chmod, open, s_isblk, s_ischr, s_isdir, s_isfifo, s_islnk, s_isreg, s_issock, statx,
};
pub use structures::{statx_timestamp, Statx};
pub use types::mode_t;
