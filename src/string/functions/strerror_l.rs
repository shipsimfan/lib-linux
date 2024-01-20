use crate::locale::locale_t;
use std::ffi::{c_char, c_int};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, EINVAL},
    string::strerror,
};

extern "C" {
    /// [`strerror_l`] is like [`strerror`], but maps `errnum` to a locale-dependent error message in
    /// the locale specified by locale. The behavior of [`strerror_l`] is undefined if locale is
    /// the special locale object [`LC_GLOBAL_LOCALE`] or is not a valid locale object handle.
    ///
    /// # Return Value
    /// The [`strerror_l`] function returns the appropriate error description string, or an
    /// "Unknown error nnn" message if the error number is unknown.
    ///
    /// POSIX.1-2001 and POSIX.1-2008 require that a successful call to [`strerror_l`] shall leave
    /// [`errno`] unchanged, and note that, since no function return value is reserved to indicate
    /// an error, an application that wishes to check for errors should initialize [`errno`] to
    /// zero before the call, and then check [`errno`] after the call.
    ///
    /// # Errors
    ///  * [`EINVAL`] - The value of `errnum` is not a valid error number.
    pub fn strerror_l(errnum: c_int, local: locale_t) -> *mut c_char;
}
