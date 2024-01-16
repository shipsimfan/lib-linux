use crate::sys::stat::mode_t;
use std::ffi::{c_char, c_int};

// rustdoc imports
#[allow(unused_imports)]
use crate::errno::errno;

extern "C" {
    /// [`chmod`] changes the permissions of the file specified whose pathname is given in `path`,
    /// which is dereferenced if it is a symbolic link.
    ///
    /// # Return Value
    /// On success, zero is returned. On error, -1 is returned, and [`errno`] is set appropriately.
    pub fn chmod(path: *const c_char, mode: mode_t) -> c_int;
}
