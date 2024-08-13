use std::ffi::c_int;

/// A timezone
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct timezone {
    /// Minutes west of Greenwich
    pub minuteswest: c_int,

    /// Type of DST correction
    pub dsttime: c_int,
}
