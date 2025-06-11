use crate::c_size_t;
use core::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::stdlib::free;
#[allow(unused_imports)]
use std::ptr::null_mut;

extern "C" {
    /// Allocate zeroed dynamic memory
    ///
    /// # Description
    /// The [`calloc`] function allocates memory for an array of `nmemb` elements of `size` bytes
    /// each and returns a pointer to the allocated memory. The memory is set to zero. If `nmemb`
    /// or `size` is 0, then [`calloc`] returns either [`null_mut`], or a unique pointer value that
    /// can later be successfully passed to [`free`].
    ///
    /// # Return Value
    /// The [`calloc`] function returns a pointer to the allocated memory that is suitably aligned
    /// for any kind of variable. On error, [`null_mut`] is returned. [`null_mut`] may also be
    /// returned by a successful call to [`calloc`] with `nmemb` or `size` equal to zero.
    pub fn calloc(nmemb: c_size_t, size: c_size_t) -> *mut c_void;
}
