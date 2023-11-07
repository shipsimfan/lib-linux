use core::ffi::{c_char, c_int, c_size_t};

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::errno;

extern "C" {
    /// strerror, strerror_r - return string describing error number
    ///
    /// ## Description
    /// The [`strerror_r`] function returns a pointer to a string that describes the error code
    /// passed in the argument errnum, possibly using the [`LC_MESSAGES`] part of the current
    /// locale to select the appropriate language. (For example, if errnum is [`EINVAL`], the
    /// returned description will "Invalid argument".) This string must not be modified by the
    /// application, but may be modified by a subsequent call to [`strerror_r`]. No library
    /// function, including [`perror`], will modify this string.
    ///
    /// ## Return Value
    /// A successful call to [`strerror_r`] shall leave [`errno`] unchanged, and note that, since
    /// no function return value is reserved to indicate an error, an application that wishes to
    /// check for errors should initialize [`errno`] to zero before the call, and then check
    /// [`errno`] after the call.
    pub fn strerror_r(err_num: c_int, buf: *mut c_char, buf_len: c_size_t) -> c_int;
}
