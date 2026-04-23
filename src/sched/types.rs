use std::ffi::c_ulong;

// rustdoc imports
#[allow(unused_imports)]
use crate::sched::cpu_set_t;

/// Type for array elements in [`cpu_set_t`].
#[allow(non_camel_case_types)]
pub type __cpu_mask = c_ulong;
