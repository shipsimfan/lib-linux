use std::ffi::c_int;

/// Socket address is intended for `bind`.
pub const AI_PASSIVE: c_int = 0x0001;

/// Request for canonical name.
pub const AI_CANONNAME: c_int = 0x0002;

/// Don't use name resolution.
pub const AI_NUMERICHOST: c_int = 0x0004;

/// IPv4 mapped addresses are acceptable.
pub const AI_V4MAPPED: c_int = 0x0008;

/// Return IPv4 mapped and IPv6 addresses.
pub const AI_ALL: c_int = 0x0010;

/// Use configuration of this host to choose returned address type.
pub const AI_ADDRCONFIG: c_int = 0x0020;

/// IDN encode input (assuming it is encoded in the current locale's character set) before looking
/// it up.
pub const AI_IDN: c_int = 0x0040;

/// Translate canonical name from IDN format.
pub const AI_CANONIDN: c_int = 0x0080;

/// Don't use name resolution.
pub const AI_NUMERICSERV: c_int = 0x0400;
