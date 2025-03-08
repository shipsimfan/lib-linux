use std::ffi::c_int;

/// There is data to read
pub const POLLIN: c_int = 0x001;

/// There is urgent data to read
pub const POLLPRI: c_int = 0x002;

/// Writing now will not block
pub const POLLOUT: c_int = 0x004;

/// Error condition
pub const POLLERR: c_int = 0x008;

/// Hung up
pub const POLLHUP: c_int = 0x010;

/// Invalid polling request
pub const POLLNVAL: c_int = 0x020;
