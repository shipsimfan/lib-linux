use crate::unistd::pid_t;
use std::{
    ffi::{c_int, c_void},
    ptr::null_mut,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::signal::{SIGEV_SIGNAL, SIGEV_THREAD, SIGEV_THREAD_ID};

/// Structure for notification from asynchronous routines
#[repr(C, packed(8))]
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct sigevent {
    /// Data passed with notification
    pub value: sigval,

    /// Notification signal
    pub signo: c_int,

    /// Notification method
    pub notify: c_int,

    /// Function used for thread notification ([`SIGEV_THREAD`])
    pub un: sigevent_union,
}

/// Data passed with notification
#[repr(C, packed(8))]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub union sigval {
    /// Integer value
    pub int: c_int,

    /// Pointer value
    pub ptr: *mut c_void,
}

#[repr(C, packed(8))]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub union sigevent_union {
    pad: [c_int; SIGEV_PAD_SIZE],

    /// When [`SIGEV_SIGNAL`] and [`SIGEV_THREAD_ID`] set, LWP ID of the thread to receive the
    /// signal
    pub tid: pid_t,

    /// Used when [`SIGEV_THREAD`] is set
    pub thread: sigevent_thread,
}

#[repr(C, packed(8))]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct sigevent_thread {
    pub function: Option<extern "C" fn(sigval)>,
    pub attribute: *mut c_void,
}

const SIGEV_MAX_SIZE: usize = 64;
const SIGEV_PAD_SIZE: usize = (SIGEV_MAX_SIZE / std::mem::size_of::<c_int>()) - 4;

impl Default for sigevent {
    fn default() -> Self {
        sigevent {
            notify: 0,
            signo: 0,
            value: sigval::default(),
            un: sigevent_union::default(),
        }
    }
}

impl Default for sigval {
    fn default() -> Self {
        sigval { ptr: null_mut() }
    }
}

impl Default for sigevent_union {
    fn default() -> Self {
        sigevent_union {
            pad: [0; SIGEV_PAD_SIZE],
        }
    }
}

impl Default for sigevent_thread {
    fn default() -> Self {
        sigevent_thread {
            function: None,
            attribute: null_mut(),
        }
    }
}
