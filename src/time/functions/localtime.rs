use crate::time::{time_t, tm};

// rustdoc imports
#[allow(unused_imports)]
use crate::time::tzset;

extern "C" {
    /// Transform date and time to broken-down local time
    ///
    /// The [`localtime`] function converts the calendar time `timep` to broken-down time
    /// representation, expressed relative to the user's specified timezone. The function acts as
    /// if it called [`tzset`] and sets the external variables tzname with information about the
    /// current timezone, timezone with the difference between Coordinated Universal Time (UTC) and
    /// local standard time in seconds, and daylight to a nonzero value if daylight savings time
    /// rules apply during some part of the year. The return value points to a statically allocated
    /// struct which might be overwritten by subsequent calls to any of the date and time
    /// functions.
    pub fn localtime(timep: *const time_t) -> *mut tm;
}
