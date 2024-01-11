use crate::{Locale, LC_ALL_MASK};
use raw::{errno::errno, string::strerror_l};
use std::ffi::{c_int, CStr};

/// Result from a linux function that can error
pub type Result<T> = std::result::Result<T, Error>;

/// An error that occured while calling a linux function
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Error(c_int);

const EMPTY_LOCALE: &'static CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"\0") };

impl Error {
    /// Creates an error from the current value of [`errno`]
    pub fn errno() -> Self {
        Error(*unsafe { errno() })
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { *errno() = 0 };

        let locale = Locale::new(LC_ALL_MASK, EMPTY_LOCALE, None).unwrap();

        let ptr = unsafe { strerror_l(self.0, locale.inner()) };

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
        write!(f, "linux::Error({})", self.0)
    }
}
