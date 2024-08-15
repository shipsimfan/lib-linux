use crate::time::time_t;

// rustdoc imports
#[allow(unused_imports)]
use crate::errno::errno;
#[allow(unused_imports)]
use std::ptr::null_mut;

extern "C" {
    /// Get time in seconds
    ///
    /// # Description
    /// [`time`] returns the time as the number of seconds since the Epoch, 1970-01-01 00:00:00
    /// +0000 (UTC).
    ///
    /// If `t` is not [`null_mut`], the return value is also stored in the memory pointed to by
    /// `t`.
    ///
    /// # Return Value
    /// On success, the value of time in seconds since the Epoch is returned. On error,
    /// `-1 as time_t` is returned, and [`errno`] is set appropriately.
    pub fn time(t: *mut time_t) -> time_t;
}
