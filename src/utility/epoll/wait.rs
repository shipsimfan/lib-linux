use crate::{
    sys::epoll::{epoll_event, epoll_wait},
    try_linux, EPoll, Result,
};

impl EPoll {
    /// Wait for an I/O event on an epoll file descriptor
    pub fn wait(&self, events: &mut [epoll_event], timeout: Option<usize>) -> Result<usize> {
        try_linux!(epoll_wait(
            self.handle,
            events.as_mut_ptr(),
            events.len() as _,
            timeout.map(|timeout| timeout as _).unwrap_or(-1)
        ))
        .map(|num| num as _)
    }
}
