use std::ffi::{c_long, c_uint};

/// [`uid_t`] is a type used to hold user IDs. It is an integer type.
#[allow(non_camel_case_types)]
pub type uid_t = c_uint;

/// [`off_t`] is used for describing file sizes. It is a signed integer type.
#[allow(non_camel_case_types)]
pub type off_t = c_long;
