use crate::c_size_t;
use core::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::stdlib::{calloc, free, malloc};
#[allow(unused_imports)]
use std::ptr::null_mut;

extern "C" {
    /// Reallocate dynamic memory
    ///
    /// # Description
    /// The [`realloc`] function changes the size of the memory block pointed to by `ptr` to `size`
    /// bytes. The contents will be unchanged in the range from the start of the region up to the
    /// minimum of the old and new sizes. If the new size is larger than the old size, the added
    /// memory will not be initialized. If `ptr` is [`null_mut`], then the call is equivalent to
    /// `malloc(size)`, for all values of `size`; if `size` is equal to zero, and `ptr` is not
    /// [`null_mut`], then the call is equivalent to `free(ptr)`. Unless `ptr` is [`null_mut`], it
    /// must have been returned by an earlier call to [`malloc`], [`calloc`] or [`realloc`]. If the
    /// area pointed to was moved, a `free(ptr)` is done.
    ///
    /// # Return Value
    /// The [`realloc`] function returns a pointer to the newly allocated memory, which is suitably
    /// aligned for any kind of variable and may be different from `ptr`, or [`null_mut`] if the
    /// request fails. If `size` was equal to 0, either [`null_mut`] or a pointer suitable to be
    /// passed to [`free`] is returned. If [`realloc`] fails the original block is left untouched;
    /// it is not freed or moved.
    pub fn realloc(ptr: *mut c_void, size: c_size_t) -> *mut c_void;
}
