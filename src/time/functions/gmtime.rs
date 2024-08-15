use crate::time::{time_t, tm};

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

extern "C" {
    /// Transform date and time to broken-down UTC time
    ///
    /// The [`gmtime`] function converts the calendar time `timep` to broken-down time
    /// representation, expressed in Coordinated Universal Time (UTC). It may return [`null_mut`]
    /// when the year does not fit into an integer. The return value points to a statically
    /// allocated struct which might be overwritten by subsequent calls to any of the date and time
    /// functions.
    pub fn gmtime(timep: *const time_t) -> *mut tm;
}
