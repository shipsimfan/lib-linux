use crate::Error;
use std::ffi::c_int;

impl Error {
    /// Gets the underlying error value
    pub fn get(&self) -> c_int {
        self.error
    }
}
