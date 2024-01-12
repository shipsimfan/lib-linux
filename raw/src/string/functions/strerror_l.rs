use crate::locale::locale_t;
use std::ffi::{c_char, c_int};

// rustdoc imports
#[allow(unused_imports)]
use crate::errno::errno;

extern "C" {
    /// strerror_l - return string describing error number
    ///
    /// The [`strerror`] function returns a pointer to a string that describes the error code
    /// passed in the argument `errnum`, possibly using the [`LC_MESSAGES`] part of the current
    /// locale to select the appropriate language. (For example, if errnum is [`EINVAL`], the
    /// returned description will "Invalid argument".) This string must not be modified by the
    /// application, but may be modified by a subsequent call to [`strerror`] or [`strerror_l`]. No
    /// library function, including [`perror`], will modify this string.
    ///
    /// [`strerror_l`] is like [`strerror`], but maps errnum to a locale-dependent error message in
    /// the locale specified by locale. The behavior of [`strerror_l`] is undefined if locale is
    /// the special locale object [`LC_GLOBAL_LOCALE`] or is not a valid locale object handle.
    ///
    /// # Return Value
    pub fn strerror_l(err_num: c_int, local: locale_t) -> *mut c_char;
}
