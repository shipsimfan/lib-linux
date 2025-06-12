use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::dlfcn::dlopen;

/// Unix98 demands the following flag which is the inverse to [`RTLD_GLOBAL`]. The implementation
/// does this by default and so we can define the value to zero.
pub const RTLD_LOCAL: c_int = 0;

/// Lazy function call binding
pub const RTLD_LAZY: c_int = 0x00001;

/// Immediate function call binding
pub const RTLD_NOW: c_int = 0x00002;

/// Do not load the object
pub const RTLD_NOLOAD: c_int = 0x00004;

/// Use deep binding
pub const RTLD_DEEPBIND: c_int = 0x00008;

/// If the following bit is set in the `flags` argument to [`dlopen`], the symbols of the loaded
/// object and its dependencies are made visible as if the object were linked directly into the
/// program.
pub const RTLD_GLOBAL: c_int = 0x00100;

/// Do not delete object when closed
pub const RTLD_NODELETE: c_int = 0x01000;
