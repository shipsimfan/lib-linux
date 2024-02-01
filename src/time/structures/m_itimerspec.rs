use super::m_timespec::timespec;

/// Describes the initial expiration of a timer, and its interval, in seconds and nanoseconds.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct itimerspec {
    /// The interval between repeats
    pub interval: timespec,

    /// The initial expiration
    pub value: timespec,
}

impl Default for itimerspec {
    fn default() -> Self {
        itimerspec {
            interval: timespec::default(),
            value: timespec::default(),
        }
    }
}
