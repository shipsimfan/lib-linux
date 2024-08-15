use core::ffi::c_int;

/// Broken down time
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[cfg(not(target_env = "gnu"))]
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

#[cfg(not(target_env = "gnu"))]
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

/// Broken down time
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[cfg(target_env = "gnu")]
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

    /// Seconds east of UTC
    pub gmtoff: core::ffi::c_long,

    /// Timezone abbreviation
    pub zone: *const core::ffi::c_char,
}

#[cfg(target_env = "gnu")]
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
            gmtoff: 0,
            zone: std::ptr::null(),
        }
    }
}
