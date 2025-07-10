use crate::{
    sys::epoll::{epoll_ctl, epoll_event, EPOLL_CTL_ADD, EPOLL_CTL_DEL},
    try_linux, AsHandle, EPoll, Result,
};
use std::ptr::null;

impl EPoll {
    /// Register a file descriptor with epoll
    pub fn add<T: AsHandle>(&mut self, fd: &T, event: &epoll_event) -> Result<()> {
        try_linux!(epoll_ctl(self.handle, EPOLL_CTL_ADD, fd.as_handle(), event)).map(|_| ())
    }

    /// Modify the events associated with an already registered file descriptor
    pub fn modify<T: AsHandle>(&mut self, fd: &T, event: &epoll_event) -> Result<()> {
        try_linux!(epoll_ctl(self.handle, EPOLL_CTL_ADD, fd.as_handle(), event)).map(|_| ())
    }

    /// Deregister a file descriptor with epoll
    pub fn remove<T: AsHandle>(&mut self, fd: &T) -> Result<()> {
        try_linux!(epoll_ctl(
            self.handle,
            EPOLL_CTL_DEL,
            fd.as_handle(),
            null()
        ))
        .map(|_| ())
    }
}
