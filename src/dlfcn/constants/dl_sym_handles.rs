use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::dlfcn::dlsym;

/// If the first argument to [`dlsym`] or [`dlvsym`] is set to [`RTLD_DEFAULT`] the run-time
/// address of the symbol called `name` in the global scope is returned.
pub const RTLD_DEFAULT: *mut c_void = 0 as _;

/// If the first argument of [`dlsym`] or [`dlvsym`] is set to [`RTLD_NEXT`] the run-time address
/// of the symbol called `name` in the next shared object is returned.  The "next" relation is
/// defined by the order the shared objects were loaded.
pub const RTLD_NEXT: *mut c_void = usize::MAX as _;
