use std::ffi::c_int;

/// Wait for all events to complete
pub const LIO_WAIT: c_int = 0;

/// Do not wait for events to complete
pub const LIO_NOWAIT: c_int = 1;
