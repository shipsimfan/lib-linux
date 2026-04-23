use crate::{EPoll, Result, sys::epoll::epoll_create, try_linux};
use std::ffi::c_int;

impl EPoll {
    /// Creates a new [`EPoll`]
    pub fn new() -> Result<Self> {
        try_linux!(epoll_create(1)).map(|handle| EPoll { handle })
    }

    /// Creates a new [`EPoll`] from a raw `handle`
    pub unsafe fn from_raw(handle: c_int) -> EPoll {
        EPoll { handle }
    }
}
