use std::ffi::c_int;

/// Broken down time
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct tm {
    /// Seconds
    pub sec: c_int,

    /// Minutes
    pub min: c_int,

    /// Hours
    pub hour: c_int,

    /// Day of the month
    pub mday: c_int,

    /// Month
    pub mon: c_int,

    /// Year
    pub year: c_int,

    /// Day of the week
    pub wday: c_int,

    /// Day in the year
    pub yday: c_int,

    /// Daylight saving time
    pub isdst: c_int,
}

impl Default for tm {
    fn default() -> Self {
        tm {
            sec: 0,
            min: 0,
            hour: 0,
            mday: 0,
            mon: 0,
            year: 0,
            wday: 0,
            yday: 0,
            isdst: 0,
        }
    }
}
