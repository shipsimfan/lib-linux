/// Input event type
pub const EPOLLIN: u32 = 0x001;

/// Exceptional event type
pub const EPOLLPRI: u32 = 0x002;

/// Output event type
pub const EPOLLOUT: u32 = 0x004;

/// Normal read event type
pub const EPOLLRDNORM: u32 = 0x040;

/// Out-of-band read event type
pub const EPOLLRDBAND: u32 = 0x080;

/// Normal write event type
pub const EPOLLWRNORM: u32 = 0x100;

/// Out-of-band write event type
pub const EPOLLWRBAND: u32 = 0x200;

/// Message event type
pub const EPOLLMSG: u32 = 0x400;

/// Error condition event type
pub const EPOLLERR: u32 = 0x008;

/// Hang-up event type
pub const EPOLLHUP: u32 = 0x010;

/// Remote hang-up event type
pub const EPOLLRDHUP: u32 = 0x2000;
