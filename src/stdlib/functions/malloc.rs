use crate::c_size_t;
use core::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::stdlib::free;
#[allow(unused_imports)]
use std::ptr::null_mut;

extern "C" {
    /// Allocate dynamic memory
    ///
    /// # Description
    /// The [`malloc`] function allocates `size` bytes and returns a pointer to the allocated
    /// memory. The memory is not initialized. If `size` is 0, then [`malloc`] returns either
    /// [`null_mut`], or a unique pointer value that can later be successfully passed to [`free`].
    ///
    /// # Return Value
    /// The [`malloc`] function returns a pointer to the allocated memory that is suitably aligned
    /// for any kind of variable. On error, [`null_mut`] is returned. [`null_mut`] may also be
    /// returned by a successful call to [`malloc`] with a `size` of zero.
    pub fn malloc(size: c_size_t) -> *mut c_void;
}
