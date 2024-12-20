use std::ffi::c_int;

/// Get file status flags
pub const F_GETFL: c_int = 3;

/// Set file status flags
pub const F_SETFL: c_int = 4;
