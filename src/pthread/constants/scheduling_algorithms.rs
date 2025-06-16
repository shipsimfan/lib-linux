use std::ffi::c_int;

/// The default scheduling algorithm
pub const SCHED_OTHER: c_int = 0;

/// First-in first-out scheduling algorithm
pub const SCHED_FIFO: c_int = 1;

/// Round-robin scheduling algorithm
pub const SCHED_RR: c_int = 2;
