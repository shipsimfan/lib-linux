use std::ffi::c_int;

/// Don't delay send to coalesce packets  
pub const TCP_NODELAY: c_int = 1;

/// Set maximum segment size  
pub const TCP_MAXSEG: c_int = 2;

/// Control sending of partial frames  
pub const TCP_CORK: c_int = 3;

/// Start keeplives after this period
pub const TCP_KEEPIDLE: c_int = 4;

/// Interval between keepalives
pub const TCP_KEEPINTVL: c_int = 5;

/// Number of keepalives before death
pub const TCP_KEEPCNT: c_int = 6;

/// Number of SYN retransmits
pub const TCP_SYNCNT: c_int = 7;

/// Life time of orphaned FIN-WAIT-2 state
pub const TCP_LINGER2: c_int = 8;

/// Wake up listener only when data arrive
pub const TCP_DEFER_ACCEPT: c_int = 9;

/// Bound advertised window
pub const TCP_WINDOW_CLAMP: c_int = 10;

/// Information about this connection.
pub const TCP_INFO: c_int = 11;

/// Bock/re-enable quick ACKs.  
pub const TCP_QUICKACK: c_int = 12;

/// Congestion control algorithm.  
pub const TCP_CONGESTION: c_int = 13;

/// TCP MD5 Signature (RFC2385)
pub const TCP_MD5SIG: c_int = 14;

/// TCP Cookie Transactions
pub const TCP_COOKIE_TRANSACTIONS: c_int = 15;

/// Use linear timeouts for thin streams
pub const TCP_THIN_LINEAR_TIMEOUTS: c_int = 16;

/// Fast retrans. after 1 dupack
pub const TCP_THIN_DUPACK: c_int = 17;

/// How long for loss retry before timeout
pub const TCP_USER_TIMEOUT: c_int = 18;

/// TCP sock is under repair right now
pub const TCP_REPAIR: c_int = 19;

/// Set TCP queue to repair
pub const TCP_REPAIR_QUEUE: c_int = 20;

/// Set sequence number of repaired queue.
pub const TCP_QUEUE_SEQ: c_int = 21;

/// Repair TCP connection options
pub const TCP_REPAIR_OPTIONS: c_int = 22;

/// Enable FastOpen on listeners
pub const TCP_FASTOPEN: c_int = 23;

/// TCP time stamp
pub const TCP_TIMESTAMP: c_int = 24;

/// Limit number of unsent bytes in write queue.  
pub const TCP_NOTSENT_LOWAT: c_int = 25;

/// Get Congestion Control (optional) info.  
pub const TCP_CC_INFO: c_int = 26;

/// Record SYN headers for new connections.  
pub const TCP_SAVE_SYN: c_int = 27;

/// Get SYN headers recorded for connection.  
pub const TCP_SAVED_SYN: c_int = 28;

/// Get/set window parameters.  
pub const TCP_REPAIR_WINDOW: c_int = 29;

/// Attempt FastOpen with connect.  
pub const TCP_FASTOPEN_CONNECT: c_int = 30;

/// Attach a ULP to a TCP connection.  
pub const TCP_ULP: c_int = 31;

/// TCP MD5 Signature with extensions.  
pub const TCP_MD5SIG_EXT: c_int = 32;

/// Set the key for Fast Open (cookie).  
pub const TCP_FASTOPEN_KEY: c_int = 33;

/// Enable TFO without a TFO cookie.  
pub const TCP_FASTOPEN_NO_COOKIE: c_int = 34;

/// Notify bytes available to readas a cmsg on read.  
pub const TCP_INQ: c_int = 36;
