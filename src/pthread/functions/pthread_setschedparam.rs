use crate::pthread::{pthread_t, sched_param};
use std::ffi::c_int;

#[link(name = "pthread")]
extern "C" {
    /// Set scheduling policy and parameters of a thread
    ///
    /// The [`pthread_setschedparam`] function sets the scheduling policy and parameters of the
    /// thread `thread`.
    ///
    /// `policy` specifies the new scheduling policy for thread.
    ///
    /// The structure pointed to by `param` specifies the new scheduling parameters for thread.
    ///
    /// # Return Value
    /// On success, this function returns 0; on error, it returns a nonzero error number. If
    /// [`pthread_setschedparam`] fails, the scheduling policy and parameters of thread are not
    /// changed.
    pub fn pthread_setschedparam(
        thread: pthread_t,
        policy: c_int,
        param: *const sched_param,
    ) -> c_int;
}
