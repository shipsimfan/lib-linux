use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::sys::socket::listen;

/// Maximum queue length specifiable by [`listen`]
pub const SOMAXCONN: c_int = 4096;
