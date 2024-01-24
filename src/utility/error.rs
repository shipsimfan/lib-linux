use crate::{errno::errno, string::strerror_r};
use std::ffi::{c_int, CStr};

/// A specialized result for Linux errors
pub type Result<T> = core::result::Result<T, Error>;

/// An error reported by Linux
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Error(c_int);

/// Convert a linux system call result (-1 on error) into a [`Result<c_int>`]
#[macro_export]
macro_rules! try_linux {
    ($expr: expr) => {
        match unsafe { $expr } {
            -1 => Err($crate::LinuxError::errno()),
            result => Ok(result),
        }
    };
}

impl Error {
    /// Creates a new [`Error`]
    pub const fn new(error: c_int) -> Self {
        Error(error)
    }

    /// Creates a new [`Error`] from the current value of [`errno`]
    pub fn errno() -> Self {
        Error::new(unsafe { *errno() })
    }

    /// Gets the underlying error value
    pub fn get(&self) -> c_int {
        self.0
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = [0; 64];

        let message =
            unsafe { CStr::from_ptr(strerror_r(self.0, buffer.as_mut_ptr(), buffer.len())) };

        write!(f, "{}", message.to_string_lossy())
    }
}
