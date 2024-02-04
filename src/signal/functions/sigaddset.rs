use crate::signal::sigset_t;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, EINVAL},
    signal::{sigemptyset, sigfillset},
};

extern "C" {
    /// The [`sigaddset`] function adds the individual signal specified by the `signo` to the
    /// signal set pointed to by `set`.
    ///
    /// Applications shall call either [`sigemptyset`] or [`sigfillset`] at least once for each
    /// object of type [`sigset_t`] prior to any other use of that object. If such an object is not
    /// initialized in this way, but is nonetheless supplied as an argument to [`sigaddset`], the
    /// results are undefined.
    ///
    /// # Return Value
    /// Upon successful completion, [`sigaddset`] shall return 0; otherwise, it shall return -1 and
    /// set [`errno`] to indicate the error.
    ///
    /// # Errors
    ///  * [`EINVAL`] - The value of the `signo` argument is an invalid or unsupported signal number.
    pub fn sigaddset(set: *mut sigset_t, signo: c_int) -> c_int;
}
