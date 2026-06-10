use std::ffi::c_void;

/// Return value of `mmap' in case of an error.
pub const MAP_FAILED: *mut c_void = usize::MAX as _;
