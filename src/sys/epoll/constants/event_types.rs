use std::ffi::c_int;

/// Input event type
pub const EPOLLIN: c_int = 0x001;

/// Exceptional event type
pub const EPOLLPRI: c_int = 0x002;

/// Output event type
pub const EPOLLOUT: c_int = 0x004;

/// Normal read event type
pub const EPOLLRDNORM: c_int = 0x040;

/// Out-of-band read event type
pub const EPOLLRDBAND: c_int = 0x080;

/// Normal write event type
pub const EPOLLWRNORM: c_int = 0x100;

/// Out-of-band write event type
pub const EPOLLWRBAND: c_int = 0x200;

/// Message event type
pub const EPOLLMSG: c_int = 0x400;

/// Error condition event type
pub const EPOLLERR: c_int = 0x008;

/// Hang-up event type
pub const EPOLLHUP: c_int = 0x010;

/// Remote hang-up event type
pub const EPOLLRDHUP: c_int = 0x2000;
