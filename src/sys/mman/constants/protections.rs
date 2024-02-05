use std::ffi::c_int;

/// Page cannot be accessed
pub const PROT_NONE: c_int = 0x0;

/// Page can be read
pub const PROT_READ: c_int = 0x1;

/// Page can be written
pub const PROT_WRITE: c_int = 0x2;

/// Page can be executed
pub const PROT_EXEC: c_int = 0x4;
