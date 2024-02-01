use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::fcntl::{O_CLOEXEC, O_NONBLOCK};

/// See [`O_CLOEXEC`]
pub const SFD_CLOEXEC: c_int = 0o2000000;

/// See [`O_NONBLOCK`]
pub const SFD_NONBLOCK: c_int = 0o0004000;
