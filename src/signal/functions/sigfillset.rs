use crate::signal::sigset_t;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::errno::errno;

extern "C" {
    /// The [`sigfillset`] function shall initialize the signal set pointed to by set, such that
    /// all signals defined in this volume of POSIX.1‐2017 are included.
    ///
    /// # Return Value
    /// Upon successful completion, [`sigfillset`] shall return 0; otherwise, it shall return -1
    /// and set [`errno`] to indicate the error.
    ///
    /// # Errors
    /// No errors are defined.
    pub fn sigfillset(set: *mut sigset_t) -> c_int;
}
