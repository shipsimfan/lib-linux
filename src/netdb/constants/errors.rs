use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::errno::errno;

/// Invalid value for `flags` field.
pub const EAI_BADFLAGS: c_int = -1;

/// NAME or SERVICE is unknown.
pub const EAI_NONAME: c_int = -2;

/// Temporary failure in name resolution.
pub const EAI_AGAIN: c_int = -3;

/// Non-recoverable failure in name resolution.
pub const EAI_FAIL: c_int = -4;

/// No address associated with NAME.
pub const EAI_NODATA: c_int = -5;

/// `family` not supported.
pub const EAI_FAMILY: c_int = -6;

/// `socktype` not supported.
pub const EAI_SOCKTYPE: c_int = -7;

/// `service` not supported for `socktype`.
pub const EAI_SERVICE: c_int = -8;

/// Address family for `name` not supported.
pub const EAI_ADDRFAMILY: c_int = -9;

/// Memory allocation failure.
pub const EAI_MEMORY: c_int = -10;

/// System error returned in [`errno`].
pub const EAI_SYSTEM: c_int = -11;

/// Argument buffer overflow.
pub const EAI_OVERFLOW: c_int = -12;

/// Processing request in progress.
pub const EAI_INPROGRESS: c_int = -100;

/// Request canceled.
pub const EAI_CANCELED: c_int = -101;

/// Request not canceled.
pub const EAI_NOTCANCELED: c_int = -102;

/// All requests done.
pub const EAI_ALLDONE: c_int = -103;

/// Interrupted by a signal.
pub const EAI_INTR: c_int = -104;

/// IDN encoding failed.
pub const EAI_IDN_ENCODE: c_int = -105;
