use std::ffi::c_int;

/// The time is an absolute time, not relative
pub const TFD_TIMER_ABSTIME: c_int = 1 << 0;

/// Cancel the timer if the clock changes
pub const TFD_TIMER_CANCEL_ON_SET: c_int = 1 << 1;

/// Close the timer on [`exec`]
pub const TFD_CLOEXEC: c_int = 0o2000000;

/// Set the timer to be non-blocking
pub const TFD_NONBLOCK: c_int = 0o0004000;
