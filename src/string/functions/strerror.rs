use std::ffi::{c_char, c_int};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, EINVAL, ERANGE},
    locale::LC_MESSAGES,
};

extern "C" {
    /// The [`strerror`] function returns a pointer to a string that describes the error code
    /// passed in the argument `errnum`, possibly using the [`LC_MESSAGES`] part of the current
    /// locale to select the appropriate language. (For example, if `errnum` is [`EINVAL`], the
    /// returned description will "Invalid argument".) This string must not be modified by the
    /// application, but may be modified by a subsequent call to [`strerror`]. No library function,
    /// including [`perror`], will modify this string.
    ///
    /// # Return Value
    /// The [`strerror`] function returns the appropriate error description string, or an "Unknown
    /// error nnn" message if the error number is unknown.
    ///
    /// POSIX.1-2001 and POSIX.1-2008 require that a successful call to [`strerror`] shall leave
    /// [`errno`] unchanged, and note that, since no function return value is reserved to indicate
    /// an error, an application that wishes to check for errors should initialize [`errno`] to
    /// zero before the call, and then check [`errno`] after the call.
    ///
    /// # Errors
    ///  * [`EINVAL`] - The value of `errnum` is not a valid error number.
    pub fn strerror(errnum: c_int) -> *const c_char;
}
