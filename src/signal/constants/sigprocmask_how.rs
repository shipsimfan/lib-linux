use std::ffi::c_int;

/// Block signals
pub const SIG_BLOCK: c_int = 0;

/// Unblock signals
pub const SIG_UNBLOCK: c_int = 1;

/// Set the set of blocked signals
pub const SIG_SETMASK: c_int = 2;
