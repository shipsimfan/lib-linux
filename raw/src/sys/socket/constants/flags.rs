use std::ffi::c_int;

/// Process out-of-band data
pub const MSG_OOB: c_int = 0x01;

/// Peek at incoming messages
pub const MSG_PEEK: c_int = 0x02;

/// Don't use local routing
pub const MSG_DONTROUTE: c_int = 0x04;

/// Same as [`MSG_DONTROUTE`]
pub const MSG_TRYHARD: c_int = MSG_DONTROUTE;

/// Control data lost before delivery
pub const MSG_CTRUNC: c_int = 0x08;

/// Supply or ask for second address
pub const MSG_PROXY: c_int = 0x10;

#[allow(missing_docs)]
pub const MSG_TRUNC: c_int = 0x20;

/// Nonblocking IO
pub const MSG_DONTWAIT: c_int = 0x40;

/// End of record
pub const MSG_EOR: c_int = 0x80;

/// Wait for a full request
pub const MSG_WAITALL: c_int = 0x100;

#[allow(missing_docs)]
pub const MSG_FIN: c_int = 0x200;
#[allow(missing_docs)]
pub const MSG_SYN: c_int = 0x400;

/// Confirm path validity
pub const MSG_CONFIRM: c_int = 0x800;

#[allow(missing_docs)]
pub const MSG_RST: c_int = 0x1000;

/// Fetch message from error queue
pub const MSG_ERRQUEUE: c_int = 0x2000;

/// Do not generate SIGPIPE
pub const MSG_NOSIGNAL: c_int = 0x4000;

/// Sender will send more
pub const MSG_MORE: c_int = 0x8000;

/// Wait for at least one packet to return
pub const MSG_WAITFORONE: c_int = 0x10000;

/// More messages coming
pub const MSG_BATCH: c_int = 0x40000;

/// Use user data in kernel path
pub const MSG_ZEROCOPY: c_int = 0x4000000;

/// Send data in TCP SYN
pub const MSG_FASTOPEN: c_int = 0x20000000;

/// Set `close_on_exit` for file
pub const MSG_CMSG_CLOEXEC: c_int = 0x40000000;
