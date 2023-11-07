use crate::{raw, AddressFamily, Descriptor, Error, Result, SocketType};
use std::ffi::c_int;

/// A socket created with a call to [`raw::socket`]
#[derive(PartialEq, Eq)]
pub struct Socket {
    handle: c_int,
}

impl Socket {
    /// # Socket
    /// Creates a new [`Socket`]
    ///
    /// ## Paramters
    ///  * `domain` - The [`AddressFamily`] that the socket will operate on.
    ///  * `r#type` - The [`SocketType`] that the socket will act as.
    ///  * `protocol` - The protocol for this socket to operate on, usually the only option is 0.
    ///
    /// ## Return Value
    /// Returns a new [`Socket`] on success or the error that occurred while trying to create it.
    ///
    /// ## Remarks
    /// See [`raw::socket`] for more information on this function
    pub fn new(domain: AddressFamily, r#type: SocketType, protocol: i32) -> Result<Self> {
        let handle = unsafe { raw::socket(domain as c_int, r#type as c_int, protocol) };
        if handle == -1 {
            Err(Error::errno())
        } else {
            Ok(Socket { handle })
        }
    }
}

impl Descriptor for Socket {
    unsafe fn as_raw_handle(&self) -> c_int {
        self.handle
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        unsafe { raw::close(self.handle) };
    }
}
