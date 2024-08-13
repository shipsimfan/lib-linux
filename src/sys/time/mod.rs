//! Time types

mod functions;
mod structures;

pub use functions::gettimeofday;
pub use structures::{timeval, timezone};
