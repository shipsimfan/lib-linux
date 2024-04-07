//! Get and set file status

mod functions;
mod types;

pub use functions::{chmod, open};
pub use types::mode_t;
