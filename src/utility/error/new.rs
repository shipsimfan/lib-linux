use crate::{errno::errno, Error};
use std::ffi::c_int;

impl Error {
    /// Creates a new [`Error`]
    pub const fn new(error: c_int) -> Self {
        Error { error }
    }

    /// Creates a new [`Error`] from the current value of [`errno`]
    pub fn errno() -> Self {
        Error::new(unsafe { *errno() })
    }
}
