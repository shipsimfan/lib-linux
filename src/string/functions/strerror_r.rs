use core::ffi::{c_char, c_int, c_size_t};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, EINVAL, ERANGE},
    string::strerror,
};

extern "C" {
    /// The [`strerror_r`] function is similar to [`strerror`], but is thread safe. The
    /// XSI-compliant [`strerror_r`] is preferred for portable applications. It returns the error
    /// string in the user-supplied buffer `buf` of length `buflen`.
    ///
    /// # Return Value
    /// The XSI-compliant [`strerror_r`] function returns 0 on success. On error, a (positive)
    /// error number is returned (since glibc 2.13), or -1 is returned and [`errno`] is set to
    /// indicate the error (glibc versions before 2.13).
    ///
    /// # Errors
    ///  * [`EINVAL`] - The value of `errnum` is not a valid error number.
    ///  * [`ERANGE`] - Insufficient storage was supplied to contain the error string
    pub fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: c_size_t) -> c_int;
}
