use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::signal::SIGCHLD;

/// Don't send [`SIGCHLD`] when children stop
pub const SA_NOCLDSTOP: c_int = 1;

/// Don't create zombie on child death
pub const SA_NOCLDWAIT: c_int = 2;

/// Invoke signal-catching function with three arguments instead of one
pub const SA_SIGINFO: c_int = 4;

/// Use signal stack by using `sa_restorer`
pub const SA_ONSTACK: c_int = 0x08000000;

/// See [`SA_ONSTACK`]
pub const SA_STACK: c_int = SA_ONSTACK;

/// Restart syscall on signal return
pub const SA_RESTART: c_int = 0x10000000;

/// Historical no-op
pub const SA_INTERRUPT: c_int = 0x20000000;

/// Don't automatically block the signal when its handler is being executed
pub const SA_NODEFER: c_int = 0x40000000;

/// See [`SA_NODEFER`]
pub const SA_NOMASK: c_int = SA_NODEFER;

/// Reset to [`SIG_DFL`] on entry to handler
pub const SA_RESETHAND: c_int = -0x80000000;

/// See [`SA_RESETHAND`]
pub const SA_ONESHOT: c_int = SA_RESETHAND;
