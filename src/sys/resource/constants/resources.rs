use std::ffi::c_int;

/// Per-process CPU limit, in seconds.
pub const RLIMIT_CPU: c_int = 0;

/// Largest file that can be created, in bytes.
pub const RLIMIT_FSIZE: c_int = 1;

/// Maximum size of data segment, in bytes.
pub const RLIMIT_DATA: c_int = 2;

/// Maximum size of stack segment, in bytes.
pub const RLIMIT_STACK: c_int = 3;

/// Largest core file that can be created, in bytes.
pub const RLIMIT_CORE: c_int = 4;

/// Largest resident set size, in bytes. This affects swapping; processes that are exceeding their
/// resident set size will be more likely to have physical memory taken from them.
pub const RLIMIT_RSS: c_int = 5;

/// Number of open files.
pub const RLIMIT_NOFILE: c_int = 7;

/// Address space limit.
pub const RLIMIT_AS: c_int = 9;

/// Number of processes.
pub const RLIMIT_NPROC: c_int = 6;

/// Locked-in-memory address space.
pub const RLIMIT_MEMLOCK: c_int = 8;

/// Maximum number of file locks.
pub const RLIMIT_LOCKS: c_int = 10;

/// Maximum number of pending signals.
pub const RLIMIT_SIGPENDING: c_int = 11;

/// Maximum bytes in POSIX message queues.
pub const RLIMIT_MSGQUEUE: c_int = 12;

/// Maximum nice priority allowed to raise to. Nice levels 19 .. -20 correspond to 0 .. 39 values
/// of this resource limit.
pub const RLIMIT_NICE: c_int = 13;

/// Maximum realtime priority allowed for non-priviledged processes.
pub const RLIMIT_RTPRIO: c_int = 14;

/// Maximum CPU time in microseconds that a process scheduled under a real-time scheduling policy
/// may consume without making a blocking system call before being forcibly descheduled.
pub const RLIMIT_RTTIME: c_int = 15;
