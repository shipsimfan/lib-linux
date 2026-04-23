use std::ffi::c_int;

/// Seek from beginning of file.
pub const SEEK_SET: c_int = 0;

/// Seek from current position.
pub const SEEK_CUR: c_int = 1;

/// Seek from end of file.
pub const SEEK_END: c_int = 2;

/// Seek to next data.
pub const SEEK_DATA: c_int = 3;

/// Seek to next hole.
pub const SEEK_HOLE: c_int = 4;
