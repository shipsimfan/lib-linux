use crate::time::{suseconds_t, time_t};

/// A point in time
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct timeval {
    /// Seconds
    pub sec: time_t,

    /// Microseconds
    pub usec: suseconds_t,
}
