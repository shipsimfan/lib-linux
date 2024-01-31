use std::ffi::c_ulong;

/// A set of signals to be blocked, unblocked, or waited for
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct sigset_t {
    /// Internal value, should not be accessed from the outside
    _val: [c_ulong; _SIGSET_NWORDS],
}

/// The size of the signal set
const _SIGSET_NWORDS: usize = 1024 / (8 * std::mem::size_of::<c_ulong>());

impl Default for sigset_t {
    fn default() -> Self {
        sigset_t {
            _val: [0; _SIGSET_NWORDS],
        }
    }
}
