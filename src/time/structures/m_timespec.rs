use crate::time::time_t;
use std::ffi::c_long;

/// Time in seconds and nanoseconds
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct timespec {
    /// Number of seconds
    pub sec: time_t,

    /// Number of nanoseconds
    pub nsec: c_long,
}

impl Default for timespec {
    fn default() -> Self {
        timespec { sec: 0, nsec: 0 }
    }
}
