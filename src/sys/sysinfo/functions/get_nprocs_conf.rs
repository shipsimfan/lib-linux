use std::ffi::c_int;

extern "C" {
    /// The function [`get_nprocs_conf`] returns the number of processors configured by the
    /// operating system.
    pub fn get_nprocs_conf() -> c_int;
}
