use crate::{sys::eventfd::eventfd, try_linux, EventFd, Result};
use std::ffi::c_int;

impl EventFd {
    /// Creates a new [`EventFd`]
    pub fn new(initial_value: usize, flags: c_int) -> Result<Self> {
        try_linux!(eventfd(initial_value as _, flags)).map(|handle| EventFd { handle })
    }

    /// Creates a new [`EventFd`] from a raw `handle`
    pub unsafe fn from_raw(handle: c_int) -> EventFd {
        EventFd { handle }
    }
}
