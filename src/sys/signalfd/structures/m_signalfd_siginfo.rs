// rustdoc imports
#[allow(unused_imports)]
use crate::unistd::read;

/// Structure returned by [`read`]s to a signalfd
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct signalfd_siginfo {
    /// Signal number
    pub signo: u32,

    /// Error number
    pub errno: i32,

    /// Signal code
    pub code: i32,

    /// PID of sender
    pub pid: u32,

    /// Real UID of sender
    pub uid: u32,

    /// File descriptor
    pub fd: i32,

    /// Kernel timer ID
    pub tid: u32,

    /// Band event
    pub band: u32,

    /// POSIX timer overrun count
    pub overrun: u32,

    /// Trap number that caused signal
    pub trapno: u32,

    /// Exit status or signal
    pub status: i32,

    /// Integer sent
    pub int: i32,

    /// Pointer sent
    pub ptr: u64,

    /// User CPU time consumed
    pub utime: u64,

    /// System CPU time consumed
    pub stime: u64,

    /// Address that generated signal
    pub addr: u64,

    /// Least significant bit of address
    pub addr_lsb: u16,

    /// Pad to 4 byte alignment
    _pad2: u16,

    /// Syscall requested
    pub syscall: i32,

    /// The address of the call
    pub call_addr: u64,

    /// The architecture for the call
    pub arch: u32,

    /// Pad size to 128 bytes
    _pad: [u8; 28],
}

impl Default for signalfd_siginfo {
    fn default() -> Self {
        signalfd_siginfo {
            signo: 0,
            errno: 0,
            code: 0,
            pid: 0,
            uid: 0,
            fd: 0,
            tid: 0,
            band: 0,
            overrun: 0,
            trapno: 0,
            status: 0,
            int: 0,
            ptr: 0,
            utime: 0,
            stime: 0,
            addr: 0,
            addr_lsb: 0,
            _pad2: 0,
            syscall: 0,
            call_addr: 0,
            arch: 0,
            _pad: [0; 28],
        }
    }
}
