use crate::signal::sigset_t;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::signal::sigprocmask;

#[link(name = "pthread")]
extern "C" {
    /// The [`pthread_sigmask`] function is just like [`sigprocmask`], with the difference that its
    /// use in multithreaded programs is explicitly specified by POSIX.1.
    ///
    /// For a description of the arguments and operation of this function, see [`sigprocmask`].
    ///
    /// # Return Value
    /// On success, [`pthread_sigmask`] returns 0; on error, it returns an error number.
    ///
    /// # Errors
    /// See [`sigprocmask`].
    pub fn pthread_sigmask(how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int;
}
