use std::ffi::c_char;

// rustdoc imports
#[allow(unused_imports)]
use crate::dlfcn::dlopen;
#[allow(unused_imports)]
use std::ptr::null_mut;

extern "C" {
    /// Obtain error diagnostic for functions in the [`dlopen`] API
    ///
    /// The [`dlerror`] function returns a human-readable, null-terminated string describing the
    /// most recent error that occurred from a call to one of the functions in the [`dlopen`] API
    /// since the last call to [`dlerror`]. The returned string does not include a trailing
    /// newline.
    ///
    /// [`dlerror`] returns [`null_mut`] if no errors have occurred since initialization or since
    /// it was last called.
    pub fn dlerror() -> *mut c_char;
}
