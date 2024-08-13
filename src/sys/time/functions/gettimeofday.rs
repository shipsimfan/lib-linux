use crate::sys::time::{timeval, timezone};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "c")]
extern "C" {
    /// Gets the time as well as a timezone.
    ///
    /// If either `tv` or `tz` is [`null_mut`], the corresponding structure is not set or returned.
    ///
    /// The use of the [`timezone`] structure is obsolete; the `tz` argument should normally be
    /// specified as [`null_mut`].  
    pub fn gettimeofday(tv: *mut timeval, tz: *mut timezone) -> c_int;
}
