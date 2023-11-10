use crate::{
    raw::{self, socklen_t},
    AddressFamily, Descriptor, Error, Result, SocketAddress, SocketType,
};
use std::{
    ffi::c_int,
    mem::{size_of, MaybeUninit},
};

/// A socket created with a call to [`raw::socket`]
#[derive(PartialEq, Eq)]
pub struct Socket {
    handle: c_int,
}

impl Socket {
    /// Creates a new [`Socket`]
    ///
    /// ## Paramters
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

    /// Sends a message on this socket
    ///
    /// ## Parameters
    ///  * `buffer` - The data to send
    ///  * `flags` - The set of flags used for this message. See [`raw::send`] for more info.
    ///
    /// ## Return Value
    /// Returns the number of bytes sent on success or the error that occurred while trying to
    /// send.
    ///
    /// ## Remarks
    /// See [`raw::send`] for more information on this function
    pub fn send(&self, buffer: &[u8], flags: i32) -> Result<usize> {
        let result = unsafe { raw::send(self.handle, buffer.as_ptr().cast(), buffer.len(), flags) };
        if result == -1 {
            Err(Error::errno())
        } else {
            Ok(result as usize)
        }
    }

    /// Sends a message to a target on this socket
    ///
    /// ## Parameters
    ///  * `buffer` - The data to send
    ///  * `flags` - The set of flags used for this message. See [`raw::sendto`] for more info.
    ///  * `address` - The target to send this message to
    ///
    /// ## Return Value
    /// Returns the number of bytes sent on success or the error that occurred while trying to
    /// send.
    ///
    /// ## Remarks
    /// See [`raw::sendto`] for more information on this function
    pub fn send_to<S: SocketAddress>(
        &self,
        buffer: &[u8],
        flags: i32,
        address: &S,
    ) -> Result<usize> {
        let result = unsafe {
            raw::sendto(
                self.handle,
                buffer.as_ptr().cast(),
                buffer.len(),
                flags,
                address.as_sockaddr_ptr(),
                size_of::<S>() as socklen_t,
            )
        };
        if result == -1 {
            Err(Error::errno())
        } else {
            Ok(result as usize)
        }
    }

    /// Receives a message on this socket
    ///
    /// ## Parameters
    ///  * `buffer` - The buffer for the message to be read into
    ///  * `flags` - The set of flags used for this message. See [`raw::recv`] for more info.
    ///
    /// ## Return Value
    /// Returns the number of received read on success or the error that occurred while trying to
    /// recieve.
    ///
    /// ## Remarks
    /// See [`raw::recv`] for more information on this function
    pub fn recv(&self, buffer: &mut [u8], flags: i32) -> Result<usize> {
        let result =
            unsafe { raw::recv(self.handle, buffer.as_mut_ptr().cast(), buffer.len(), flags) };
        if result == -1 {
            Err(Error::errno())
        } else {
            Ok(result as usize)
        }
    }

    /// Receives a message on this socket
    ///
    /// ## Parameters
    ///  * `buffer` - The buffer for the message to be read into
    ///  * `flags` - The set of flags used for this message. See [`raw::recv`] for more info.
    ///
    /// ## Return Value
    /// Returns the number of received read on success or the error that occurred while trying to
    /// recieve.
    ///
    /// ## Remarks
    /// See [`raw::recv`] for more information on this function
    pub fn recv_from<S: SocketAddress>(
        &self,
        buffer: &mut [u8],
        flags: i32,
    ) -> Result<(usize, Option<S>)> {
        let mut size = 0;
        let mut address = MaybeUninit::uninit();
        let result = unsafe {
            raw::recvfrom(
                self.handle,
                buffer.as_mut_ptr().cast(),
                buffer.len(),
                flags,
                address.as_mut_ptr() as _,
                &mut size,
            )
        };
        if result == -1 {
            return Err(Error::errno());
        }

        if size as usize >= size_of::<S>() {
            let address: S = unsafe { address.assume_init() };

            if address.family() == S::FAMILY {
                return Ok((result as usize, Some(address)));
            }
        }

        Ok((result as usize, None))
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
