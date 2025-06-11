/// Equivalent to C's `size_t` type, from `stddef.h` (or `cstddef` for C++).
#[allow(non_camel_case_types)]
pub type c_size_t = usize;

/// Equivalent to C's `ssize_t` (on POSIX) or `SSIZE_T` (on Windows) type.
#[allow(non_camel_case_types)]
pub type c_ssize_t = isize;
