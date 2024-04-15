// rustdoc imports
#[allow(unused_imports)]
use crate::sys::stat::statx;

/// Timestamp returned by the [`statx`] system call
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct statx_timestamp {
    /// The number of seconds
    pub sec: i64,

    /// The nanoseconds
    pub nsec: u32,

    /// Padding
    pub pad: [i32; 1],
}

impl Default for statx_timestamp {
    fn default() -> Self {
        statx_timestamp {
            sec: 0,
            nsec: 0,
            pad: [0; 1],
        }
    }
}
