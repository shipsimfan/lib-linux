use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null;

extern "C" {
    fn __errno_location() -> *mut c_int;
}

/// errno - number of last error
///
/// # Description
/// The <errno.h> header file defines the integer variable [`errno`], which is set by system calls
/// and some library functions in the event of an error to indicate what went wrong.
///
/// The value in [`errno`] is significant only when the return value of the call indicated an error
/// (i.e., -1 from most system calls; -1 or [`null`] from most library functions); a function that
/// succeeds is allowed to change [`errno`]. The value of [`errno`] is never set to zero by any
/// system call or library function.
///
/// For some system calls and library functions (e.g., [`getpriority`]), -1 is a valid return on
/// success. In such cases, a successful return can be distinguished from an error return by
/// setting [`errno`] to zero before the call, and then, if the call returns a status that
/// indicates that an error may have occurred, checking to see if errno has a nonzero value.
///
/// [`errno`] is defined by the ISO C standard to be a modifiable lvalue of type int, and must not
/// be explicitly declared; [`errno`] may be a macro. errno is thread-local; setting it in one
/// thread does not affect its value in any other thread.
pub unsafe fn errno() -> &'static mut c_int {
    &mut *__errno_location()
}
