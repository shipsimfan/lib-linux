use crate::signal::{siginfo_t, sigset_t};
use std::ffi::{c_int, c_void};

// rustdoc imports
#[allow(unused_imports)]
use crate::signal::SA_SIGINFO;

/// Structure describing the action to be taken when a signal arrives
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct sigaction_t {
    /// Signal handler
    pub handler: sigaction_handler,

    /// Additional set of signals to be blocked
    pub mask: sigset_t,

    /// Special flags
    pub flags: c_int,

    /// Restore handler
    pub restorer: Option<extern "C" fn()>,
}

/// Signal handler
#[repr(C)]
#[derive(Clone, Copy)]
pub union sigaction_handler {
    /// Used if [`SA_SIGINFO`] is not set
    pub handler: isize,

    /// Used if [`SA_SIGINFO`] is set
    pub sigaction: Option<extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
}

impl Default for sigaction_t {
    fn default() -> Self {
        sigaction_t {
            handler: sigaction_handler::default(),
            mask: sigset_t::default(),
            flags: 0,
            restorer: None,
        }
    }
}

impl Default for sigaction_handler {
    fn default() -> Self {
        sigaction_handler { handler: 0 }
    }
}
