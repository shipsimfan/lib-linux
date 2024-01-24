//! Timers that notify via file descriptors

mod constants;
mod functions;

pub use constants::*;
pub use functions::{timerfd_create, timerfd_gettime, timerfd_settime};
