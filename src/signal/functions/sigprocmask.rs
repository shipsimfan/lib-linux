use crate::signal::sigset_t;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, EFAULT, EINVAL},
    signal::{SIG_BLOCK, SIG_SETMASK, SIG_UNBLOCK},
};
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

extern "C" {
    /// [`sigprocmask`] is used to fetch and/or change the signal mask of the calling thread. The
    /// signal mask is the set of signals whose delivery is currently blocked for the caller.
    ///
    /// The behavior of the call is dependent on the value of how, as follows.
    ///  * [`SIG_BLOCK`] - The set of blocked signals is the union of the current set and the set
    ///                    argument.
    ///  * [`SIG_UNBLOCK`] - The signals in set are removed from the current set of blocked
    ///                      signals. It is permissible to attempt to unblock a signal which is not
    ///                      blocked.
    ///  * [`SIG_SETMASK`] - The set of blocked signals is set to the argument set.
    ///
    /// If `oldset` is not [`null_mut`], the previous value of the signal mask is stored in
    /// `oldset`.
    ///
    /// If `set` is [`null`], then the signal mask is unchanged (i.e., how is ignored), but the
    /// current value of the signal mask is nevertheless returned in `oldset` (if it is not
    /// [`null_mut`]).
    ///
    /// The use of [`sigprocmask`] is unspecified in a multithreaded process; see
    /// [`pthread_sigmask`].
    ///
    /// # Return Value
    /// [`sigprocmask`] returns 0 on success. On failure, -1 is returned and [`errno`] is set to
    /// indicate the error.
    ///
    /// # Errors
    ///  * [`EFAULT`] - The `set` or `oldset` argument points outside the process's allocated
    ///                 address space.
    ///  * [`EINVAL`] - The value specified in `how` was invalid
    pub fn sigprocmask(how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int;
}
