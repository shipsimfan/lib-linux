use std::{ffi::c_int, os::fd::AsRawFd};

/// An item which wraps a linux handle
pub trait AsHandle {
    /// Get the underlying linux handle
    fn as_handle(&self) -> c_int;
}

impl AsHandle for c_int {
    fn as_handle(&self) -> c_int {
        *self
    }
}

impl<T: AsHandle> AsHandle for &T {
    fn as_handle(&self) -> c_int {
        T::as_handle(self)
    }
}

impl<T: AsHandle> AsHandle for Box<T> {
    fn as_handle(&self) -> c_int {
        T::as_handle(self)
    }
}

impl<T: AsHandle> AsHandle for std::rc::Rc<T> {
    fn as_handle(&self) -> c_int {
        T::as_handle(self)
    }
}

impl<'a, T: AsHandle> AsHandle for std::cell::Ref<'a, T> {
    fn as_handle(&self) -> c_int {
        T::as_handle(self)
    }
}

impl<'a, T: AsHandle> AsHandle for std::cell::RefMut<'a, T> {
    fn as_handle(&self) -> c_int {
        T::as_handle(self)
    }
}

impl<T: AsHandle> AsHandle for std::sync::Arc<T> {
    fn as_handle(&self) -> c_int {
        T::as_handle(self)
    }
}

impl<'a, T: AsHandle> AsHandle for std::sync::MutexGuard<'a, T> {
    fn as_handle(&self) -> c_int {
        T::as_handle(self)
    }
}

impl<'a, T: AsHandle> AsHandle for std::sync::RwLockReadGuard<'a, T> {
    fn as_handle(&self) -> c_int {
        T::as_handle(self)
    }
}

impl<'a, T: AsHandle> AsHandle for std::sync::RwLockWriteGuard<'a, T> {
    fn as_handle(&self) -> c_int {
        T::as_handle(self)
    }
}

impl AsHandle for std::net::TcpListener {
    fn as_handle(&self) -> c_int {
        self.as_raw_fd()
    }
}

impl AsHandle for std::net::TcpStream {
    fn as_handle(&self) -> c_int {
        self.as_raw_fd()
    }
}

impl AsHandle for std::net::UdpSocket {
    fn as_handle(&self) -> c_int {
        self.as_raw_fd()
    }
}

impl AsHandle for std::fs::File {
    fn as_handle(&self) -> c_int {
        self.as_raw_fd()
    }
}

impl AsHandle for std::io::Stdout {
    fn as_handle(&self) -> c_int {
        self.as_raw_fd()
    }
}

impl AsHandle for std::io::Stderr {
    fn as_handle(&self) -> c_int {
        self.as_raw_fd()
    }
}

impl AsHandle for std::io::Stdin {
    fn as_handle(&self) -> c_int {
        self.as_raw_fd()
    }
}

impl<'a> AsHandle for std::io::StdoutLock<'a> {
    fn as_handle(&self) -> c_int {
        self.as_raw_fd()
    }
}

impl<'a> AsHandle for std::io::StderrLock<'a> {
    fn as_handle(&self) -> c_int {
        self.as_raw_fd()
    }
}

impl<'a> AsHandle for std::io::StdinLock<'a> {
    fn as_handle(&self) -> c_int {
        self.as_raw_fd()
    }
}
