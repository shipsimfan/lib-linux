use crate::{
    signal::sigevent,
    time::{clockid_t, timer_t},
};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, EAGAIN, EINVAL, ENOMEM, EOPNOTSUPP, EPERM},
    signal::{siginfo_t, SIGALRM, SIGEV_NONE, SIGEV_SIGNAL, SIGEV_THREAD, SIGEV_THREAD_ID},
    time::{
        CLOCK_BOOTTIME, CLOCK_BOOTTIME_ALARM, CLOCK_MONOTONIC, CLOCK_PROCESS_CPUTIME_ID,
        CLOCK_REALTIME, CLOCK_REALTIME_ALARM, CLOCK_TAI, CLOCK_THREAD_CPUTIME_ID,
    },
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "rt")]
extern "C" {
    /// [`timer_create`] creates a new per-process interval timer. The ID of the new timer is
    /// returned in the buffer pointed to by `timerid`, which must be a non-null pointer. This ID
    /// is unique within the process, until the timer is deleted. The new timer is initially
    /// disarmed.
    ///
    /// The `clockid` argument specifies the clock that the new timer uses to measure time. It can
    /// be specified as one of the following values:    
    ///  * [`CLOCK_REALTIME`] - A settable system-wide real-time clock.
    ///  * [`CLOCK_MONOTONIC`] - A nonsettable monotonically increasing clock that measures time
    ///                           from some unspecified point in the past that does not change
    ///                           after system startup.
    ///  * [`CLOCK_PROCESS_CPUTIME_ID`] - (since Linux 2.6.12) A clock that measures (user and
    ///                                    system) CPU time consumed by (all of the threads in) the
    ///                                    calling process.
    ///  * [`CLOCK_THREAD_CPUTIME_ID`] - (since Linux 2.6.12) A clock that measures (user and
    ///                                   system) CPU time consumed by the calling thread.
    ///  * [`CLOCK_BOOTTIME`] - (Since Linux 2.6.39) Like [`CLOCK_MONOTONIC`], this is a
    ///                          monotonically increasing clock. However, whereas the
    ///                          [`CLOCK_MONOTONIC`] clock does not measure the time while a system
    ///                          is suspended, the [`CLOCK_BOOTTIME`] clock does include the time
    ///                          during which the system is suspended. This is useful for
    ///                          applications that need to be suspend-aware. [`CLOCK_REALTIME`] is
    ///                          not suitable for such applications, since that clock is affected
    ///                          by discontinuous changes to the system clock.
    ///  * [`CLOCK_REALTIME_ALARM`] - This clock is like [`CLOCK_REALTIME`], but will wake the
    ///                                system if it is suspended. The caller must have the
    ///                                [`CAP_WAKE_ALARM`] capability in order to set a timer
    ///                                against this clock.
    ///  * [`CLOCK_BOOTTIME_ALARM`] - This clock is like [`CLOCK_BOOTTIME`], but will wake the
    ///                                system if it is suspended. The caller must have the
    ///                                [`CAP_WAKE_ALARM`] capability in order to set a timer
    ///                                against this clock.
    ///  * [`CLOCK_TAI`] - A system-wide clock derived from wall-clock time but ignoring leap
    ///                     seconds.
    ///
    /// See [`clock_getres`] for some further details on the above clocks.
    ///
    /// As well as the above values, `clockid` can be specified as the clockid returned by a call
    /// to [`clock_getcpuclockid`] or [`pthread_getcpuclockid`].
    ///
    /// The `sevp` argument points to a sigevent structure that specifies how the caller should be
    /// notified when the timer expires.
    ///
    /// The `sevp.notify` field can have the following values:
    ///  * [`SIGEV_NONE`] - Don't asynchronously notify when the timer expires. Progress of the
    ///                     timer can be monitored using [`timer_gettime`].
    ///  * [`SIGEV_SIGNAL`] - Upon timer expiration, generate the signal `sevp.signo` for the
    ///                       process. The `code` field of the [`siginfo_t`] structure will be set
    ///                       to [`SI_TIMER`]. At any point in time, at most one signal is queued
    ///                       to the process for a given timer; see [`timer_getoverrun`] for more
    ///                       details.
    ///  * [`SIGEV_THREAD`] - Upon timer expiration, invoke `sevp.notify_function` as if it were
    ///                       the start function of a new thread.
    ///  * [`SIGEV_THREAD_ID`] - (Linux-specific) As for [`SIGEV_SIGNAL`], but the signal is
    ///                          targeted at the thread whose ID is given in
    ///                          `sevp.notify_thread_id`, which must be a thread in the same
    ///                          process as the caller. The `sevp.notify_thread_id` field specifies
    ///                          a kernel thread ID, that is, the value returned by [`clone`] or
    ///                          [`gettid`]. This flag is intended only for use by threading
    ///                          libraries.
    ///
    /// Specifying `sevp` as [`null_mut`] is equivalent to specifying a pointer to a [`sigevent`]
    /// structure in which `sevp.notify` is [`SIGEV_SIGNAL`], `sevp.signo` is [`SIGALRM`], and
    /// `sevp.value.int` is the timer ID.
    ///
    /// # Return Value
    /// On success, [`timer_create`] returns 0, and the ID of the new timer is placed in
    /// `*timerid`. On failure, -1 is returned, and [`errno`] is set to indicate the error.
    ///
    /// # Errors
    ///  * [`EAGAIN`] - Temporary error during kernel allocation of timer structures.
    ///  * [`EINVAL`] - `clockid`, `sevp.notify`, `sevp.signo`, or `sevp.notify_thread_id` is
    ///                 invalid.
    ///  * [`ENOMEM`] - Could not allocate memory.
    ///  * [`EOPNOTSUPP`] - The kernel does not support creating a timer against this `clockid`.
    ///  * [`EPERM`] -  `clockid` was [`CLOCK_REALTIME_ALARM`] or [`CLOCK_BOOTTIME_ALARM`] but the
    ///                 caller did not have the [`CAP_WAKE_ALARM`] capability.
    pub fn timer_create(clockid: clockid_t, sevp: *mut sigevent, timerid: *mut timer_t) -> c_int;
}
