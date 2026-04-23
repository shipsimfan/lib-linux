use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::errno::{EFAULT, EMFILE, ENFILE, errno};

unsafe extern "C" {
    /// Create pipe
    ///
    /// # Description
    /// [`pipe`] creates a pipe, a unidirectional data channel that can be used for interprocess
    /// communication. The array `pipefd` is used to return two file descriptors referring to the
    /// ends of the pipe. `pipefd[0]` refers to the read end of the pipe. `pipefd[1]` refers to the
    /// write end of the pipe. Data written to the write end of the pipe is buffered by the kernel
    /// until it is read from the read end of the pipe.
    ///
    /// # Return Value
    /// On success, zero is returned. On error, -1 is returned, and [`errno`] is set appropriately.
    ///
    /// # Errors
    ///  * [`EFAULT`] - `pipefd` is not valid.
    ///  * [`EMFILE`] - Too many file descriptors are in use by the process.
    ///  * [`ENFILE`] - The system limit on the total number of open files has been reached.
    pub fn pipe(pipefd: *mut [c_int; 2]) -> c_int;
}
