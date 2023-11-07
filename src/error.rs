use crate::raw;
use core::ffi::{c_int, c_size_t};
use std::ffi::CStr;

/// Result from a linux function that can error
pub type Result<T> = std::result::Result<T, Error>;

/// An error that occured while calling a linux function
pub struct Error(c_int);

impl Error {
    /// Creates an error from the current value of [`raw::errno`]
    pub fn errno() -> Self {
        Error(*unsafe { raw::errno() })
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { *raw::errno() = 0 };

        let mut buffer = [0; 256];
        unsafe { raw::strerror_r(self.0, buffer.as_mut_ptr().cast(), buffer.len() as c_size_t) };

        if unsafe { *raw::errno() } == 0 {
            if let Ok(cstr) = CStr::from_bytes_until_nul(&buffer) {
                return write!(f, "{}", cstr.to_string_lossy());
            }
        }

        write!(f, "unknown error ({})", self.0)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
