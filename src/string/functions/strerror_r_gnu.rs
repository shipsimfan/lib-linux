use core::ffi::{c_char, c_int, c_size_t};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, EINVAL, ERANGE},
    string::strerror,
};

extern "C" {
    /// The [`strerror_r`] function is similar to [`strerror`], but is thread safe. The
    /// GNU-specific [`strerror_r`] returns a pointer to a string containing the error message.
    /// This may be either a pointer to a string that the function stores in `buf`, or a pointer to
    /// some (immutable) static string (in which case `buf` is unused). If the function stores a
    /// string in `buf`, then at most `buflen` bytes are stored (the string may be truncated if
    /// `buflen` is too small and `errnum` is unknown). The string always includes a terminating
    /// null byte.
    ///
    /// # Return Value
    /// The [`strerror_r`] function returns the appropriate error description string, or an
    /// "Unknown error nnn" message if the error number is unknown.
    ///
    /// # Errors
    ///  * [`EINVAL`] - The value of `errnum` is not a valid error number.
    ///  * [`ERANGE`] - Insufficient storage was supplied to contain the error string.
    pub fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: c_size_t) -> *mut c_char;
}
