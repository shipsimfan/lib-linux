use crate::unistd::pid_t;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{errno::errno, signal::SIGCONT};

extern "C" {
    /// Send signal to a process
    ///
    /// The [`kill`] system call can be used to send any signal to any process group or process.
    ///
    /// If `pid` is positive, then signal `sig` is sent to the process with the ID specified by
    /// `pid`.
    ///
    /// If `pid` equals 0, then `sig` is sent to every process in the process group of the calling
    /// process.
    ///
    /// If `pid` equals -1, then `sig` is sent to every process for which the calling process has
    /// permission to send signals, except for process 1 (init), but see below.
    ///
    /// If `pid` is less than -1, then sig is sent to every process in the process group whose ID
    /// is -pid.
    ///
    /// If `sig` is 0, then no signal is sent, but existence and permission checks are still
    /// performed; this can be used to check for the existence of a process ID or process group ID
    /// that the caller is permitted to signal.
    ///
    /// For a process to have permission to send a signal, it must either be privileged (under
    /// Linux: have the `CAP_KILL` capability in the user namespace of the target process), or the
    /// real or effective user ID of the sending process must equal the real or saved set- user-ID
    /// of the target process. In the case of [`SIGCONT`], it suffices when the sending and
    /// receiving processes belong to the same session.
    ///
    /// # Return Value
    /// On success (at least one signal was sent), zero is returned. On error, -1 is returned, and
    /// [`errno`] is set to indicate the error.
    pub fn kill(pid: pid_t, sig: c_int) -> c_int;
}
