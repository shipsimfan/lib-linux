use crate::time::{itimerspec, timer_t};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, EFAULT, EINVAL},
    time::{timespec, CLOCK_REALTIME, TIMER_ABSTIME},
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "rt")]
extern "C" {
    /// [`timer_settime`] arms or disarms the timer identified by `timerid`. The `new_value`
    /// argument is pointer to an [`itimerspec`] structure that specifies the new initial value and
    /// the new interval for the timer.
    ///
    /// Each of the substructures of the [`itimerspec`] structure is a [`timespec`] structure that
    /// allows a time value to be specified in seconds and nanoseconds. These time values are
    /// measured according to the clock that was specified when the timer was created by
    /// [`timer_create`].
    ///
    /// If `new_value.value` specifies a nonzero value (i.e., either subfield is nonzero), then
    /// [`timer_settime`] arms (starts) the timer, setting it to initially expire at the given
    /// time. (If the timer was already armed, then the previous settings are overwritten.) If
    /// `new_value.value` specifies a zero value (i.e., both subfields are zero), then the timer is
    /// disarmed.
    ///
    /// The `new_value.interval` field specifies the period of the timer, in seconds and
    /// nanoseconds. If this field is nonzero, then each time that an armed timer expires, the
    /// timer is reloaded from the value specified in `new_value.interval`. If `new_value.interval`
    /// specifies a zero value, then the timer expires just once, at the time specified by `value`.
    ///
    /// By default, the initial expiration time specified in `new_value.value` is interpreted
    /// relative to the current time on the timer's clock at the time of the call. This can be
    /// modified by specifying [`TIMER_ABSTIME`] in flags, in which case `new_value.value` is
    /// interpreted as an absolute value as measured on the timer's clock; that is, the timer will
    /// expire when the clock value reaches the value specified by `new_value.value`. If the
    /// specified absolute time has already passed, then the timer expires immediately, and the
    /// overrun count (see [`timer_getoverrun`]) will be set correctly.
    ///
    /// If the value of the [`CLOCK_REALTIME`] clock is adjusted while an absolute timer based on
    /// that clock is armed, then the expiration of the timer will be appropriately adjusted.
    /// Adjustments to the [`CLOCK_REALTIME`] clock have no effect on relative timers based on
    /// that clock.
    ///
    /// If `old_value` is not [`null_mut`], then it points to a buffer that is used to return the
    /// previous interval of the timer (in `old_value.interval`) and the amount of time until the
    /// timer would previously have next expired (in `old_value.value`).
    ///
    /// # Return Value
    /// On success, [`timer_settime`] returns 0. On error, -1 is returned, and [`errno`] is set to
    /// indicate the error.
    ///
    /// # Errors
    /// [`timer_settime`] may fail with the following errors:
    ///  * [`EFAULT`] - `new_value` or `old_value` is not a valid pointer.
    ///  * [`EINVAL`] - `timerid` is invalid.
    ///  * [`EINVAL`] - `new_value.value` is negative; or `new_value.value.nsec` is negative or
    ///                 greater than 999,999,999.
    pub fn timer_settime(
        timerid: timer_t,
        flags: c_int,
        new_value: *const itimerspec,
        old_value: *mut itimerspec,
    ) -> c_int;
}
