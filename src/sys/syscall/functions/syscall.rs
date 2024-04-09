use std::ffi::c_long;

// rustdoc imports
#[allow(unused_imports)]
use crate::errno::{errno, ENOSYS};

extern "C" {
    /// Indirect system call
    ///
    /// # Description
    /// [`syscall`] is a small library function that invokes the system call whose assembly
    /// language interface has the specified number with the specified arguments. Employing
    /// [`syscall`] is useful, for example, when invoking a system call that has no wrapper
    /// function in the C library.
    ///
    /// [`syscall`] saves CPU registers before making the system call, restores the registers upon
    /// return from the system call, and stores any error returned by the system call in [`errno`].
    ///
    /// # Return Value
    /// The return value is defined by the system call being invoked. In general, a 0 return value
    /// indicates success. A -1 return value indicates an error, and an error number is stored in
    /// [`errno`].
    ///
    /// # Errors
    ///  * [`ENOSYS`] - The requested system call number is not implemented.
    ///
    /// Other errors are specific to the invoked system call.
    pub fn syscall(number: c_long, ...) -> c_long;
}
