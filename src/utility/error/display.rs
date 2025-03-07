use crate::{string::strerror_r, Error};
use std::ffi::CStr;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = [0; 64];

        let message = unsafe {
            CStr::from_ptr(strerror_r(self.error, buffer.as_mut_ptr(), buffer.len()) as _)
        };

        write!(f, "{}", message.to_string_lossy())
    }
}
