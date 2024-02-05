use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::fcntl::{open, O_NONBLOCK};

/// Provide semaphore-like semantics for reads from the new file descriptor.
pub const EFD_SEMAPHORE: c_int = 0o0000001;

/// Set the [`O_NONBLOCK`] file status flag on the open file description (see [`open`]) referred to
/// by the new file descriptor
pub const EFD_NONBLOCK: c_int = 0o0004000;

/// Set the close-on-exec flag on the new file descriptor
pub const EFD_CLOEXEC: c_int = 0o2000000;
