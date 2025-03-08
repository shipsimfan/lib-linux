use std::ffi::{c_int, c_short};

/// Data structure describing a poll requeust
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct pollfd {
    /// File descriptor to poll
    pub fd: c_int,

    /// Types of events poller cares about
    pub events: c_short,

    /// Types of events that actually occurred
    pub revents: c_short,
}

impl Default for pollfd {
    fn default() -> Self {
        pollfd {
            fd: 0,
            events: 0,
            revents: 0,
        }
    }
}
