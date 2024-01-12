use crate::sys::epoll::epoll_event;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::errno::errno;

extern "C" {
    /// epoll_ctl - control interface for an epoll descriptor
    ///
    /// ## Description
    /// This system call performs control operations on the epoll instance referred to by the file
    /// descriptor `epfd`. It requests that the operation `op` be performed for the target file
    /// descriptor, `fd`.
    ///
    /// ## Return Value
    /// When successful, [`epoll_ctl`] returns zero. When an error occurs, [`epoll_ctl`] returns -1
    /// and [`errno`] is set appropriately.
    pub fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *const epoll_event) -> c_int;
}
