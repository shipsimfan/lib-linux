use std::{
    ffi::{c_int, c_long},
    os::raw::c_void,
};

/// Used for system time in clock ticks or [`CLOCKS_PER_SEC`]
#[allow(non_camel_case_types)]
pub type clock_t = c_long;

/// Clock ID used in clock and timer functions
#[allow(non_camel_case_types)]
pub type clockid_t = c_int;

/// Used for time in seconds
#[allow(non_camel_case_types)]
pub type time_t = c_long;

/// Timer ID returned by `timer_create'
#[allow(non_camel_case_types)]
pub type timer_t = *mut c_void;
