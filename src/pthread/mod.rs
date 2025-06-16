//! Threads

mod functions;
mod structures;
mod types;

pub use functions::{pthread_self, pthread_setschedparam};
pub use structures::sched_param;
pub use types::pthread_t;
