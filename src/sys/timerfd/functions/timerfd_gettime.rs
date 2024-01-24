use crate::time::itimerspec;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, EBADF, EFAULT, EINVAL},
    sys::timerfd::TFD_TIMER_ABSTIME,
};

extern "C" {
    /// [`timerfd_gettime`] returns, in `curr_value`, an [`itimerspec`] structure that contains the
    /// current setting of the timer referred to by the file descriptor `fd`.
    ///
    /// The `value` field returns the amount of time until the timer will next expire. If both
    /// fields of this structure are zero, then the timer is currently disarmed. This field always
    /// contains a relative value, regardless of whether the [`TFD_TIMER_ABSTIME`] flag was
    /// specified when setting the timer.
    ///
    /// The `interval` field returns the interval of the timer. If both fields of this structure
    /// are zero, then the timer is set to expire just once, at the time specified by `curr_value.value`.
    ///
    /// # Return Value
    /// [`timerfd_gettime`] returns 0 on success; on error it returns -1, and sets [`errno`] to
    /// indicate the error.
    ///
    /// # Errors
    /// [`timerfd_gettime`] can fail with the following errors:
    ///  * [`EBADF`] - `fd` is not a valid file descriptor.
    ///  * [`EFAULT`] - `cur_value` is not a valid pointer.
    ///  * [`EINVAL`] - `fd` is not a valid timerfd file descriptor.
    pub fn timerfd_gettime(fd: c_int, curr_value: *mut itimerspec) -> c_int;
}
