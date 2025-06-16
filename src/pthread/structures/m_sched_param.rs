use std::ffi::c_int;

/// Data structure to describe a process' schedulability
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct sched_param {
    /// The priority to use for scheduling
    sched_priority: c_int,
}
