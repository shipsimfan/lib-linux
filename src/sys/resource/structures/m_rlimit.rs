use crate::sys::resource::rlim_t;

/// Defines the hard and soft limits of a resource
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct rlimit {
    /// The current (soft) limit.
    pub rlim_cur: rlim_t,

    /// The hard limit.
    pub rlim_max: rlim_t,
}

impl Default for rlimit {
    fn default() -> Self {
        rlimit {
            rlim_cur: 0,
            rlim_max: 0,
        }
    }
}
