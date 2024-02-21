use crate::time::__kernel_time64_t;
use std::ffi::c_longlong;

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
pub struct __kernel_timespec {
    /// Seconds
    pub sec: __kernel_time64_t,

    /// Nanoseconds
    pub nsec: c_longlong,
}
