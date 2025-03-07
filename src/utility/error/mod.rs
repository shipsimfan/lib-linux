use std::ffi::c_int;

mod constants;
mod display;
mod get;
mod macros;
mod new;

/// A specialized result for Linux errors
pub type Result<T> = core::result::Result<T, Error>;

/// An error reported by Linux
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Error {
    /// The error value
    error: c_int,
}

impl std::error::Error for Error {}
