use crate::unistd::pid_t;
use std::{
    ffi::{c_int, c_void},
    ptr::null_mut,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::signal::{SIGEV_THREAD, SIGEV_THREAD_ID};

/// Structure for notification from asynchronous routines
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct sigevent {
    /// Notification method
    pub notify: c_int,

    /// Notification signal
    pub signo: c_int,

    /// Data passed with notification
    pub value: sigval,

    /// Function used for thread notification ([`SIGEV_THREAD`])
    pub notify_function: Option<extern "C" fn(sigval)>,

    /// Attributes for notification thread ([`SIGEV_THREAD`])
    pub notify_attributes: *mut c_void,

    /// ID of thread to signal ([`SIGEV_THREAD_ID`]); Linux specific
    pub notify_thread_id: pid_t,
}

/// Data passed with notification
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub union sigval {
    /// Integer value
    int: c_int,

    /// Pointer value
    ptr: *mut c_void,
}

impl Default for sigevent {
    fn default() -> Self {
        sigevent {
            notify: 0,
            signo: 0,
            value: sigval::default(),
            notify_function: None,
            notify_attributes: null_mut(),
            notify_thread_id: 0,
        }
    }
}

impl Default for sigval {
    fn default() -> Self {
        sigval { ptr: null_mut() }
    }
}
