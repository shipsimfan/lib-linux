use crate::raw;
use std::ffi::{c_int, CStr};

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

        let ptr = unsafe { raw::strerror_l(self.0, todo!()) };

        if !ptr.is_null() {
            return write!(
                f,
                "{} ({})",
                unsafe { CStr::from_ptr(ptr) }.to_string_lossy(),
                self.0
            );
        }

        write!(f, "unknown error ({})", self.0)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
