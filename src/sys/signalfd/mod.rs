//! Create a file descriptor for accepting signals

mod constants;
mod functions;
mod structures;

pub use constants::*;
pub use functions::signalfd;
pub use structures::signalfd_siginfo;
