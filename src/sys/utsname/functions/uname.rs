use crate::sys::utsname::utsname;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::errno::errno;

extern "C" {
    /// get name and information about current kernel
    ///
    /// [`uname`] returns system information in the structure pointed to by `buf`.
    ///
    /// # Return Value
    /// On success, zero is returned.  On error, -1 is returned, and [`errno`] is set to indicate
    /// the error.
    pub fn uname(buf: *mut utsname) -> c_int;
}
