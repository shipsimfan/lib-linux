use crate::signal::sigset_t;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::errno::errno;

extern "C" {
    /// The [`sigemptyset`] function initializes the signal set pointed to by set, such that all
    /// signals defined in POSIX.1‐2008 are excluded.
    ///
    /// # Return Value
    /// Upon successful completion, [`sigemptyset`] shall return 0; otherwise, it shall return -1
    /// and set [`errno`] to indicate the error.
    ///
    /// # Errors
    /// No errors are defined.
    pub fn sigemptyset(set: *mut sigset_t) -> c_int;
}
