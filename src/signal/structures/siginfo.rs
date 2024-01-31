use crate::{signal::sigval, sys::types::uid_t, time::clock_t, unistd::pid_t};
use std::{
    ffi::{c_int, c_long, c_short, c_uint},
    os::raw::c_void,
    ptr::null_mut,
};

/// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::errno,
    signal::{SIGBUS, SIGCHLD, SIGFPE, SIGILL, SIGPOLL, SIGSEGV, SIGSYS},
};

/// The signal information passed to a signal handler
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct siginfo_t {
    /// Signal number
    pub signo: c_int,

    /// If non-zero, an [`errno`] value associated with this signal
    pub errno: c_int,

    /// Signal code
    pub code: c_int,

    /// Explicit Padding
    _pad0: c_int,

    /// The fields based on `signo`
    pub fields: siginfo_t_fields,
}

/// The fields based on `signo`
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub union siginfo_t_fields {
    _pad: [c_int; _SI_PAD_SIZE],

    /// [`kill`]
    pub kill: siginfo_t_kill,

    /// POSIX.1b timers
    pub timer: siginfo_t_timer,

    /// POSIX.1b signals
    pub rt: siginfo_t_rt,

    /// [`SIGCHLD`]
    pub sigchld: siginfo_t_sigchld,

    /// [`SIGILL`], [`SIGFPE`], [`SIGSEGV`], [`SIGBUS`]
    pub sigfault: siginfo_t_sigfault,

    /// [`SIGPOLL`]
    pub sigpoll: siginfo_t_sigpoll,

    /// [`SIGSYS`]
    pub sigsys: siginfo_t_sigsys,
}

/// [`kill`]
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct siginfo_t_kill {
    /// Sending process ID
    pub pid: pid_t,

    /// Real user ID of sending process
    pub uid: uid_t,
}

/// POSIX.1b timers
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct siginfo_t_timer {
    /// Timer ID
    pub tid: c_int,

    /// Overrun count
    pub overrun: c_int,

    /// Signal value
    pub si_value: sigval,
}

/// POSIX.1b signals
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct siginfo_t_rt {
    /// Sending process ID
    pub pid: pid_t,

    /// Real user ID of sending process
    pub uid: uid_t,

    /// Signal value
    pub sigval: sigval,
}

/// [`SIGCHLD`]
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct siginfo_t_sigchld {
    /// Which child
    pub pid: pid_t,

    /// Real user ID of sending process
    pub uid: uid_t,

    /// Exit value or signal
    pub status: c_int,

    /// User time consumed
    pub utime: clock_t,

    /// System time consumed
    pub stime: clock_t,
}

/// [`SIGILL`], [`SIGFPE`], [`SIGSEGV`], [`SIGBUS`]
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct siginfo_t_sigfault {
    /// Faulting insn/memory ref
    pub addr: *mut c_void,

    /// Valid LSB of the reported address
    pub addr_lsb: c_short,

    /// Fault bounds
    bounds: siginfo_t_sigfault_bounds,
}

/// Fault bounds
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub union siginfo_t_sigfault_bounds {
    /// Used when `code == SEGV_BNDERR`
    pub addr_bnd: siginfo_t_sigfault_addr_bnd,

    /// Used when `code == SEGV_PKUERR`
    pub pkey: u32,
}

/// Used when `code == SEGV_BNDERR`
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct siginfo_t_sigfault_addr_bnd {
    /// Lower bound when address violation occurred
    pub lower: *mut c_void,

    /// Upper bound when address violation occurred
    pub upper: *mut c_void,
}

/// [`SIGPOLL`]
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct siginfo_t_sigpoll {
    /// Band event
    pub band: c_long,

    /// File descriptor
    pub fd: c_int,
}

/// [`SIGSYS`]
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct siginfo_t_sigsys {
    /// Calling user insn
    pub call_addr: *mut c_void,

    /// Triggering system call number
    pub syscall: c_int,

    /// `AUDIT_ARCH_*` of syscall
    pub arch: c_uint,
}

const _SI_MAX_SIZE: usize = 128;
const _SI_PAD_SIZE: usize = (_SI_MAX_SIZE / std::mem::size_of::<c_int>()) - 4;

impl Default for siginfo_t {
    fn default() -> Self {
        siginfo_t {
            signo: 0,
            errno: 0,
            code: 0,
            _pad0: 0,
            fields: siginfo_t_fields::default(),
        }
    }
}

impl Default for siginfo_t_fields {
    fn default() -> Self {
        siginfo_t_fields {
            _pad: [0; _SI_PAD_SIZE],
        }
    }
}

impl Default for siginfo_t_kill {
    fn default() -> Self {
        siginfo_t_kill { pid: 0, uid: 0 }
    }
}

impl Default for siginfo_t_timer {
    fn default() -> Self {
        siginfo_t_timer {
            tid: 0,
            overrun: 0,
            si_value: sigval::default(),
        }
    }
}

impl Default for siginfo_t_rt {
    fn default() -> Self {
        siginfo_t_rt {
            pid: 0,
            uid: 0,
            sigval: sigval::default(),
        }
    }
}

impl Default for siginfo_t_sigchld {
    fn default() -> Self {
        siginfo_t_sigchld {
            pid: 0,
            uid: 0,
            status: 0,
            utime: 0,
            stime: 0,
        }
    }
}

impl Default for siginfo_t_sigfault {
    fn default() -> Self {
        siginfo_t_sigfault {
            addr: null_mut(),
            addr_lsb: 0,
            bounds: siginfo_t_sigfault_bounds::default(),
        }
    }
}

impl Default for siginfo_t_sigfault_bounds {
    fn default() -> Self {
        siginfo_t_sigfault_bounds {
            addr_bnd: siginfo_t_sigfault_addr_bnd::default(),
        }
    }
}

impl Default for siginfo_t_sigfault_addr_bnd {
    fn default() -> Self {
        siginfo_t_sigfault_addr_bnd {
            lower: null_mut(),
            upper: null_mut(),
        }
    }
}

impl Default for siginfo_t_sigpoll {
    fn default() -> Self {
        siginfo_t_sigpoll { band: 0, fd: 0 }
    }
}

impl Default for siginfo_t_sigsys {
    fn default() -> Self {
        siginfo_t_sigsys {
            call_addr: null_mut(),
            syscall: 0,
            arch: 0,
        }
    }
}
