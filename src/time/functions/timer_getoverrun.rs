use crate::time::timer_t;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, EINVAL},
    signal::{SIGEV_SIGNAL, SIGEV_THREAD},
};

#[link(name = "rt")]
extern "C" {
    /// [`timer_getoverrun`] returns the "overrun count" for the timer referred to by `timerid`. An
    /// application can use the overrun count to accurately calculate the number of timer
    /// expirations that would have occurred over a given time interval. Timer overruns can occur
    /// both when receiving expiration notifications via signals ([`SIGEV_SIGNAL`]), and via
    /// threads ([`SIGEV_THREAD`]).
    ///
    /// When expiration notifications are delivered via a signal, overruns can occur as follows.
    /// Regardless of whether or not a real-time signal is used for timer notifications, the system
    /// queues at most one signal per timer. (This is the behavior specified by POSIX.1. The
    /// alternative, queuing one signal for each timer expiration, could easily result in
    /// overflowing the allowed limits for queued signals on the system.) Because of system
    /// scheduling delays, or because the signal may be temporarily blocked, there can be a delay
    /// between the time when the notification signal is generated and the time when it is
    /// delivered (e.g., caught by a signal handler) or accepted (e.g., using [`sigwaitinfo`]). In
    /// this interval, further timer expirations may occur. The timer overrun count is the number
    /// of additional timer expirations that occurred between the time when the signal was
    /// generated and when it was delivered or accepted.
    ///
    /// Timer overruns can also occur when expiration notifications are delivered via invocation of
    /// a thread, since there may be an arbitrary delay between an expiration of the timer and the
    /// invocation of the notification thread, and in that delay interval, additional timer
    /// expirations may occur.
    ///
    /// # Return Value
    /// On success, [`timer_getoverrun`] returns the overrun count of the specified timer; this
    /// count may be 0 if no overruns have occurred. On failure, -1 is returned, and [`errno`] is
    /// set to indicate the error.
    ///
    /// # Errors
    ///  * [`EINVAL`] - `timerid` is not a valid timer ID.
    pub fn timer_getoverrun(timerid: timer_t) -> c_int;
}
