use std::ffi::c_int;

/// Invalid value for `ai_flags' field.
pub const EAI_BADFLAGS: c_int = -1;

/// NAME or SERVICE is unknown.
pub const EAI_NONAME: c_int = -2;

/// Temporary failure in name resolution.
pub const EAI_AGAIN: c_int = -3;

/// Non-recoverable failure in name res.
pub const EAI_FAIL: c_int = -4;

/// `ai_family' not supported.
pub const EAI_FAMILY: c_int = -6;

/// `ai_socktype' not supported.
pub const EAI_SOCKTYPE: c_int = -7;

/// SERVICE not supported for `ai_socktype'.
pub const EAI_SERVICE: c_int = -8;

/// Memory allocation failure.
pub const EAI_MEMORY: c_int = -10;

/// System error returned in `errno'.
pub const EAI_SYSTEM: c_int = -11;

/// Argument buffer overflow.
pub const EAI_OVERFLOW: c_int = -12;
