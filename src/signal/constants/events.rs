use std::ffi::c_int;

/// Notify via signal
pub const SIGEV_SIGNAL: c_int = 0;

/// Other notification: meaningless
pub const SIGEV_NONE: c_int = 1;

/// Deliver via thread creation
pub const SIGEV_THREAD: c_int = 2;

/// Send signal to specific thread
pub const SIGEV_THREAD_ID: c_int = 4;
