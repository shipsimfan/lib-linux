use std::ffi::c_int;

/// All requests were successfully canceled
pub const AIO_CANCELED: c_int = 0;

/// At least one of the requests specified was not canceled because it was in progress
pub const AIO_NOTCANCELED: c_int = 1;

/// All requests had already been completed before the call
pub const AIO_ALLDONE: c_int = 2;
