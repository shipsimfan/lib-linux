use crate::time::itimerspec;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, EBADF, ECANCELED, EFAULT, EINVAL},
    sys::timerfd::{timerfd_gettime, TFD_TIMER_ABSTIME, TFD_TIMER_CANCEL_ON_SET},
    time::{CLOCK_REALTIME, CLOCK_REALTIME_ALARM},
    unistd::read,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

extern "C" {
    /// [`timerfd_settime`] arms (starts) or disarms (stops) the timer referred to by the file
    /// descriptor `fd`.
    ///
    /// The `new_value` argument specifies the initial expiration and interval for the timer.
    ///
    /// `new_value.value` specifies the initial expiration of the timer, in seconds and
    /// nanoseconds. Setting either field of `new_value.value` to a nonzero value arms the timer.
    /// Setting both fields of `new_value.value` to zero disarms the timer.
    ///
    /// Setting one or both fields of `new_value.interval` to nonzero values specifies the period,
    /// in seconds and nanoseconds, for repeated timer expirations after the initial expiration. If
    /// both fields of `new_value.interval` are zero, the timer expires just once, at the time
    /// specified by `new_value.value`.
    ///
    /// By default, the initial expiration time specified in `new_value` is interpreted relative to
    /// the current time on the timer's clock at the time of the call (i.e., `new_value.value`
    /// specifies a time relative to the current value of the clock specified by `clockid`). An
    /// absolute timeout can be selected via the `flags` argument.
    ///
    /// The `flags` argument is a bit mask that can include the following values:
    ///  * [`TFD_TIMER_ABSTIME`] - Interpret `new_value.value` as an absolute value on the timer's
    ///                            clock. The timer will expire when the value of the timer's clock
    ///                            reaches the value specified in `new_value.value`.
    ///  * [`TFD_TIMER_CANCEL_ON_SET`] - If this flag is specified along with [`TFD_TIMER_ABSTIME`]
    ///                                  and the clock for this timer is [`CLOCK_REALTIME`] or
    ///                                  [`CLOCK_REALTIME_ALARM`], then mark this timer as
    ///                                  cancelable if the real-time clock undergoes a
    ///                                  discontinuous change ([`settimeofday`], [`clock_settime`],
    ///                                  or similar). When such changes occur, a current or future
    ///                                  [`read`] from the file descriptor will fail with the error
    ///                                  [`ECANCELED`].
    ///
    /// If the `old_value` argument is not [`null_mut`], then the [`itimerspec`] structure that it
    /// points to is used to return the setting of the timer that was current at the time of the
    /// call; see the description of [`timerfd_gettime`] following.
    ///
    /// # Return Value
    /// [`timerfd_settime`] returns 0 on success; on error it returns -1, and sets [`errno`] to
    /// indicate the error.
    ///
    /// # Errors
    /// [`timerfd_settime`] can fail with the following errors:
    ///  * [`EBADF`] - `fd` is not a valid file descriptor.
    ///  * [`EFAULT`] - `new_value` or `old_value` is not a valid pointer.
    ///  * [`EINVAL`] - `fd` is not a valid timerfd file descriptor.
    ///  * [`ECANCELED`] - The timer was cancelled
    ///  * [`EINVAL`] - `new_value` is not properly initialized (one of the `nsec` falls outside
    ///                  the range zero to 999,999,999).
    ///  * [`EINVAL`] - `flags` is invalid.
    pub fn timerfd_settime(
        fd: c_int,
        flags: c_int,
        new_value: *const itimerspec,
        old_value: *mut itimerspec,
    ) -> c_int;
}
