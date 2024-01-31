use std::ffi::c_int;

/// Error return
pub const SIG_ERR: isize = -1;

/// Default action
pub const SIG_DFL: isize = 0;

/// Ignore signal
pub const SIG_IGN: isize = 1;

/// Hangup
pub const SIGHUP: c_int = 1;

/// Interactive attention signal
pub const SIGINT: c_int = 2;

/// Quit
pub const SIGQUIT: c_int = 3;

/// Illegal instruction
pub const SIGILL: c_int = 4;

/// Trace/breakpoint trap
pub const SIGTRAP: c_int = 5;

/// Abnormal termination
pub const SIGABRT: c_int = 6;

/// Bus error
pub const SIGBUS: c_int = 7;

/// Erroneous arithmetic operation
pub const SIGFPE: c_int = 8;

/// Killed
pub const SIGKILL: c_int = 9;

/// User-defined signal 1
pub const SIGUSR1: c_int = 10;

/// Invalid access to storage
pub const SIGSEGV: c_int = 11;

/// User-defined signal 2
pub const SIGUSR2: c_int = 12;

/// Broken pipe
pub const SIGPIPE: c_int = 13;

/// Alarm clock
pub const SIGALRM: c_int = 14;

/// Termination request
pub const SIGTERM: c_int = 15;

/// Stack fault
pub const SIGSTKFLT: c_int = 16;

/// Child terminated or stopped
pub const SIGCHLD: c_int = 17;

/// Continue
pub const SIGCONT: c_int = 18;

/// Stop, unblockable
pub const SIGSTOP: c_int = 19;

/// Keyboard stop
pub const SIGTSTP: c_int = 20;

/// Background read from control terminal
pub const SIGTTIN: c_int = 21;

/// Background write to control terminal
pub const SIGTTOU: c_int = 22;

/// Urgent data is available at a socket
pub const SIGURG: c_int = 23;

/// CPU time limit exceeded
pub const SIGXCPU: c_int = 24;

/// File size limit exceeded
pub const SIGXFSZ: c_int = 25;

/// Virtual timer expired
pub const SIGVTALRM: c_int = 26;

/// Profiling timer expired
pub const SIGPROF: c_int = 27;

/// Window size change
pub const SIGWINCH: c_int = 28;

/// Pollable event occurred
pub const SIGPOLL: c_int = 29;

/// Power failure imminent
pub const SIGPWR: c_int = 30;

/// Bad system call
pub const SIGSYS: c_int = 31;

/// I/O now possible
pub const SIGIO: c_int = SIGPOLL;

/// IOT instruction
pub const SIGIOT: c_int = SIGABRT;

/// Old System V name
pub const SIGCLD: c_int = SIGCHLD;
