use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::sys::sysinfo::get_nprocs_conf;

extern "C" {
    /// The function [`get_nprocs`] returns the number of processors currently available in the
    /// system. This may be less than the number returned by [`get_nprocs_conf`] because processors
    /// may be offline (e.g., on hotpluggable systems).
    pub fn get_nprocs() -> c_int;
}
