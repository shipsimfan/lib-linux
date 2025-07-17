use crate::{
    sys::epoll::{epoll_event, epoll_wait},
    try_linux, EPoll, Result,
};

impl EPoll {
    /// Wait for I/O `events` on an epoll file descriptor
    ///
    /// This function returns the number of events that were set in `events`
    pub fn wait(&self, events: &mut [epoll_event], timeout: Option<usize>) -> Result<usize> {
        try_linux!(epoll_wait(
            self.handle,
            events.as_mut_ptr(),
            events.len() as _,
            timeout.map(|timeout| timeout as _).unwrap_or(-1)
        ))
        .map(|num| num as _)
    }

    /// Wait for an I/O `event` on an epoll file descriptor
    ///
    /// This function returns if `event` was set
    pub fn wait_one(&self, timeout: Option<usize>) -> Result<Option<epoll_event>> {
        let mut events = [epoll_event::default()];
        Ok(match self.wait(&mut events, timeout)? {
            0 => None,
            _ => Some(events[0]),
        })
    }
}
