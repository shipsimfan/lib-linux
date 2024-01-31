use std::ffi::c_long;

/// Used for time in seconds
#[allow(non_camel_case_types)]
pub type time_t = c_long;

/// Used for system time in clock ticks or [`CLOCKS_PER_SEC`]
#[allow(non_camel_case_types)]
pub type clock_t = c_long;
