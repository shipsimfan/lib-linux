//! Time types

mod constants;
mod functions;
mod structures;
mod types;

pub use constants::*;
pub use functions::{timer_create, timer_gettime, timer_settime};
pub use structures::{itimerspec, timespec};
pub use types::{clock_t, clockid_t, time_t, timer_t};
