use crate::{
    sys::epoll::{
        epoll_ctl, epoll_data_t, epoll_event, EPOLL_CTL_ADD, EPOLL_CTL_DEL, EPOLL_CTL_MOD,
    },
    try_linux, AsHandle, EPoll, Result,
};
use std::{ffi::c_void, ptr::null};

impl EPoll {
    /// Register a file descriptor with epoll
    pub fn add<T: AsHandle>(&mut self, fd: &T, event: &epoll_event) -> Result<()> {
        try_linux!(epoll_ctl(self.handle, EPOLL_CTL_ADD, fd.as_handle(), event)).map(|_| ())
    }

    /// Register a file descriptor with epoll, using a `ptr` for data
    pub fn add_ptr<T: AsHandle>(&mut self, fd: &T, events: u32, ptr: *mut c_void) -> Result<()> {
        self.add(
            fd,
            &epoll_event {
                events,
                data: epoll_data_t { ptr },
            },
        )
    }

    /// Register a file descriptor with epoll, using a `fd` for data
    pub fn add_fd<T1: AsHandle, T2: AsHandle>(
        &mut self,
        fd: &T1,
        events: u32,
        data_fd: T2,
    ) -> Result<()> {
        self.add(
            fd,
            &epoll_event {
                events,
                data: epoll_data_t {
                    fd: data_fd.as_handle(),
                },
            },
        )
    }

    /// Register a file descriptor with epoll, using a `u32` for data
    pub fn add_u32<T: AsHandle>(&mut self, fd: &T, events: u32, u32: u32) -> Result<()> {
        self.add(
            fd,
            &epoll_event {
                events,
                data: epoll_data_t { u32 },
            },
        )
    }

    /// Register a file descriptor with epoll, using a `u64` for data
    pub fn add_u64<T: AsHandle>(&mut self, fd: &T, events: u32, u64: u64) -> Result<()> {
        self.add(
            fd,
            &epoll_event {
                events,
                data: epoll_data_t { u64 },
            },
        )
    }

    /// Modify the events associated with an already registered file descriptor
    pub fn modify<T: AsHandle>(&mut self, fd: &T, event: &epoll_event) -> Result<()> {
        try_linux!(epoll_ctl(self.handle, EPOLL_CTL_MOD, fd.as_handle(), event)).map(|_| ())
    }

    /// Modify the events associated with an already registered file descriptor, using a `ptr` for data
    pub fn modify_ptr<T: AsHandle>(&mut self, fd: &T, events: u32, ptr: *mut c_void) -> Result<()> {
        self.modify(
            fd,
            &epoll_event {
                events,
                data: epoll_data_t { ptr },
            },
        )
    }

    /// Modify the events associated with an already registered file descriptor, using a `fd` for data
    pub fn modify_fd<T1: AsHandle, T2: AsHandle>(
        &mut self,
        fd: &T1,
        events: u32,
        data_fd: T2,
    ) -> Result<()> {
        self.modify(
            fd,
            &epoll_event {
                events,
                data: epoll_data_t {
                    fd: data_fd.as_handle(),
                },
            },
        )
    }

    /// Modify the events associated with an already registered file descriptor, using a `u32` for data
    pub fn modify_u32<T: AsHandle>(&mut self, fd: &T, events: u32, u32: u32) -> Result<()> {
        self.modify(
            fd,
            &epoll_event {
                events,
                data: epoll_data_t { u32 },
            },
        )
    }

    /// Modify the events associated with an already registered file descriptor, using a `u64` for data
    pub fn modify_u64<T: AsHandle>(&mut self, fd: &T, events: u32, u64: u64) -> Result<()> {
        self.modify(
            fd,
            &epoll_event {
                events,
                data: epoll_data_t { u64 },
            },
        )
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
