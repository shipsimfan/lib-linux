use crate::{sys::socket::socket, try_linux, Result, Socket};
use std::ffi::c_int;

impl Socket {
    /// Creates a new [`Socket`]
    pub fn new(domain: c_int, r#type: c_int, protocol: c_int) -> Result<Self> {
        try_linux!(socket(domain, r#type, protocol)).map(|handle| Socket { handle })
    }

    /// Creates a new [`Socket`] from a raw `handle`
    pub unsafe fn from_raw(handle: c_int) -> Socket {
        Socket { handle }
    }
}
