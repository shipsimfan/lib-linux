use std::ffi::c_int;

/// Initiate a read operation
pub const LIO_READ: c_int = 0;

/// Initiate a write operation
pub const LIO_WRITE: c_int = 1;

/// Ignore this control block
pub const LIO_NOP: c_int = 2;
