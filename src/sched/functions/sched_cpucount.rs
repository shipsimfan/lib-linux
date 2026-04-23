use crate::{c_size_t, sched::cpu_set_t};
use std::ffi::c_int;

unsafe extern "C" {
    /// Return the number of CPUs in `set`.
    pub fn __sched_cpucount(setsize: c_size_t, setp: *const cpu_set_t) -> c_int;
}
