use std::ffi::c_void;

/// A locale
#[allow(non_camel_case_types)]
pub type locale_t = *mut c_void;
