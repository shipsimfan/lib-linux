use crate::time::{itimerspec, timer_t};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, EFAULT, EINVAL},
    time::TIMER_ABSTIME,
};

#[link(name = "rt")]
extern "C" {
    /// [`timer_gettime`] returns the time until next expiration, and the interval, for the timer
    /// specified by `timerid`, in the buffer pointed to by `curr_value`. The time remaining until
    /// the next timer expiration is returned in `curr_value.value`; this is always a relative
    /// value, regardless of whether the [`TIMER_ABSTIME`] flag was used when arming the timer. If
    /// the value returned in `curr_value.value` is zero, then the timer is currently disarmed. The
    /// timer interval is returned in `curr_value.interval`. If the value returned in
    /// `curr_value.interval` is zero, then this is a "one-shot" timer.
    ///
    /// # Return Value
    /// On success, [`timer_gettime`] returns 0. On error, -1 is returned, and [`errno`] is set to
    /// indicate the error.
    ///
    /// # Errors
    /// [`timer_gettime`] may fail with the following errors:
    ///  * [`EFAULT`] - `curr_value` is not a valid pointer.
    ///  * [`EINVAL`] - `timerid` is invalid.
    pub fn timer_gettime(timerid: timer_t, curr_value: *mut itimerspec) -> c_int;
}
