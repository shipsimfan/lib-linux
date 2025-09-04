use std::ffi::{c_char, c_int};

// rustdoc imports
#[allow(unused_imports)]
use crate::netdb::getaddrinfo;

extern "C" {
    /// The [`gai_strerror`] function translates [`getaddrinfo`] error codes to a human readable
    /// string, suitable for error reporting.
    pub fn gai_strerror(errcode: c_int) -> *const c_char;
}
