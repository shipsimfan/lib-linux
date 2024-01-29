use crate::signal::sigevent;
use core::ffi::{c_int, c_size_t, c_ssize_t, c_void};
use std::ptr::null_mut;

/// Asynchronous I/O control block
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct aiocb {
    /// File descriptor
    pub filedes: c_int,

    /// Operation to be performed
    pub lio_opcode: c_int,

    /// Request priority offset
    pub reqprio: c_int,

    /// Location of buffer
    pub buf: *mut c_void,

    /// Length of transfer
    pub nbytes: c_size_t,

    /// Signal number and value
    pub sigevent: sigevent,

    // Internal members
    _next_prio: *mut aiocb,
    _abs_prio: c_int,
    _policy: c_int,
    _error_code: c_int,
    _return_value: c_ssize_t,
}

impl Default for aiocb {
    fn default() -> Self {
        aiocb {
            filedes: 0,
            lio_opcode: 0,
            reqprio: 0,
            buf: null_mut(),
            nbytes: 0,
            sigevent: sigevent::default(),
            _next_prio: null_mut(),
            _abs_prio: 0,
            _policy: 0,
            _error_code: 0,
            _return_value: 0,
        }
    }
}
