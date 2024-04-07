use core::ffi::{c_char, c_size_t};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, EACCES, EFAULT, EINVAL, ENOENT, ERANGE},
    stdlib::malloc,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

extern "C" {
    /// Get current working directory
    ///
    /// # Description
    /// This function returns a null-terminated string containing an absolute pathname that is the
    /// current working directory of the calling process. The pathname is returned as the function
    /// result and via the argument `buf`, if present.
    ///
    /// The [`getcwd`] function copies an absolute pathname of the current working directory to the
    /// array pointed to by `buf`, which is of length `size`.
    ///
    /// If the length of the absolute pathname of the current working directory, including the
    /// terminating null byte, exceeds `size` bytes, [`null_mut`] is returned, and [`errno`] is set
    /// to [`ERANGE`]; an application should check for this error, and allocate a larger buffer if
    /// necessary.
    ///
    /// As an extension to the POSIX.1-2001 standard, Linux (libc4, libc5, glibc) [`getcwd`]
    /// allocates the buffer dynamically using [`malloc`] if `buf` is [`null_mut`]. In this case,
    /// the allocated buffer has the length `size` unless `size` is zero, when `buf` is allocated
    /// as big as necessary. The caller should [`free`] the returned buffer.
    ///
    /// # Returned Value
    /// On success, this function returns a pointer to a string containing the pathname of the
    /// current working directory. This is the same value as `buf`.
    ///
    /// On failure, these functions return [`null_mut`], and [`errno`] is set to indicate the
    /// error. The contents of the array pointed to by `buf` are undefined on error.
    ///
    /// # Errors
    ///  * [`EACCES`] - Permission to read or search a component of the filename was denied.
    ///  * [`EFAULT`] - `buf` points to a bad address.
    ///  * [`EINVAL`] - The `size` argument is zero and `buf` is not [`null_mut`].
    ///  * [`ENOENT`] - The current working directory has been unlinked.
    ///  * [`ERANGE`] - The `size` argument is less than the length of the absolute pathanme of the
    ///                 working directory, including the terminating null byte. You need to
    ///                 allocate a bigger array and try again.
    pub fn getcwd(buf: *mut c_char, size: c_size_t) -> *mut c_char;
}
