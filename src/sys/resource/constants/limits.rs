use crate::sys::resource::rlim_t;

/// Value to indicate that there is no limit.
pub const RLIM_INFINITY: rlim_t = rlim_t::MAX;
