use std::ffi::c_int;

/// C Types
pub const LC_CTYPE: c_int = 0;

/// Numeric
pub const LC_NUMERIC: c_int = 1;

/// Time
pub const LC_TIME: c_int = 2;

/// Collate
pub const LC_COLLATE: c_int = 3;

/// Monetary
pub const LC_MONETARY: c_int = 4;

/// Messages
pub const LC_MESSAGES: c_int = 5;

/// All
pub const LC_ALL: c_int = 6;

/// Paper
pub const LC_PAPER: c_int = 7;

/// Name
pub const LC_NAME: c_int = 8;

/// Address
pub const LC_ADDRESS: c_int = 9;

/// Telephone
pub const LC_TELEPHONE: c_int = 10;

/// Measurement
pub const LC_MEASUREMENT: c_int = 11;

/// Identification
pub const LC_IDENTIFICATION: c_int = 12;
