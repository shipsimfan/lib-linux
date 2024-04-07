use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::stdlib::{calloc, malloc};
#[allow(unused_imports)]
use std::ptr::null_mut;

extern "C" {
    /// Free dynamic memory
    ///
    /// # Description
    /// The [`free`] function frees the memory space pointed to by `ptr`, which must have been
    /// returned by a previous call to [`malloc`], [`calloc`] or [`realloc`]. Otherwise, or if
    /// `free(ptr)` has already been called before, undefined behavior occurs. If `ptr` is
    /// [`null_mut`], no operation is performed.
    pub fn free(ptr: *mut c_void);
}
