use crate::{errno::errno, string::strerror_r};
use std::ffi::{c_int, CStr};

/// A specialized result for Linux errors
pub type Result<T> = core::result::Result<T, Error>;

/// An error reported by Linux
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Error(c_int);

/// Convert a linux system call result (-1 on error) into a [`Result<c_int>`]
#[macro_export]
macro_rules! try_linux {
    ($expr: expr) => {
        match unsafe { $expr } {
            -1 => Err($crate::Error::errno()),
            result => Ok(result),
        }
    };
}

impl Error {
    /// Creates a new [`Error`]
    pub const fn new(error: c_int) -> Self {
        Error(error)
    }

    /// Creates a new [`Error`] from the current value of [`errno`]
    pub fn errno() -> Self {
        Error::new(unsafe { *errno() })
    }

    /// Gets the underlying error value
    pub fn get(&self) -> c_int {
        self.0
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = [0; 64];

        let message =
            unsafe { CStr::from_ptr(strerror_r(self.0, buffer.as_mut_ptr(), buffer.len()) as _) };

        write!(f, "{}", message.to_string_lossy())
    }
}

// Error constants
impl Error {
    /// Operation not permitted
    pub const EPERM: Self = Error::new(crate::errno::EPERM);

    /// No such file or directory
    pub const ENOENT: Self = Error::new(crate::errno::ENOENT);

    /// No such process
    pub const ESRCH: Self = Error::new(crate::errno::ESRCH);

    /// Interrupted system call
    pub const EINTR: Self = Error::new(crate::errno::EINTR);

    /// I/O error
    pub const EIO: Self = Error::new(crate::errno::EIO);

    /// No such device or address
    pub const ENXIO: Self = Error::new(crate::errno::ENXIO);

    /// Argument list too long
    pub const E2BIG: Self = Error::new(crate::errno::E2BIG);

    /// Exec format error
    pub const ENOEXEC: Self = Error::new(crate::errno::ENOEXEC);

    /// Bad file number
    pub const EBADF: Self = Error::new(crate::errno::EBADF);

    /// No child processes
    pub const ECHILD: Self = Error::new(crate::errno::ECHILD);

    /// Try again
    pub const EAGAIN: Self = Error::new(crate::errno::EAGAIN);

    /// Out of memory
    pub const ENOMEM: Self = Error::new(crate::errno::ENOMEM);

    /// Permission denied
    pub const EACCES: Self = Error::new(crate::errno::EACCES);

    /// Bad address
    pub const EFAULT: Self = Error::new(crate::errno::EFAULT);

    /// Block device required
    pub const ENOTBLK: Self = Error::new(crate::errno::ENOTBLK);

    /// Device or resource busy
    pub const EBUSY: Self = Error::new(crate::errno::EBUSY);

    /// File exists
    pub const EEXIST: Self = Error::new(crate::errno::EEXIST);

    /// Cross-device link
    pub const EXDEV: Self = Error::new(crate::errno::EXDEV);

    /// No such device
    pub const ENODEV: Self = Error::new(crate::errno::ENODEV);

    /// Not a directory
    pub const ENOTDIR: Self = Error::new(crate::errno::ENOTDIR);

    /// Is a directory
    pub const EISDIR: Self = Error::new(crate::errno::EISDIR);

    /// Invalid argument
    pub const EINVAL: Self = Error::new(crate::errno::EINVAL);

    /// File table overflow
    pub const ENFILE: Self = Error::new(crate::errno::ENFILE);

    /// Too many open files
    pub const EMFILE: Self = Error::new(crate::errno::EMFILE);

    /// Not a typewriter
    pub const ENOTTY: Self = Error::new(crate::errno::ENOTTY);

    /// Text file busy
    pub const ETXTBSY: Self = Error::new(crate::errno::ETXTBSY);

    /// File too large
    pub const EFBIG: Self = Error::new(crate::errno::EFBIG);

    /// No space left on device
    pub const ENOSPC: Self = Error::new(crate::errno::ENOSPC);

    /// Illegal seek
    pub const ESPIPE: Self = Error::new(crate::errno::ESPIPE);

    /// Read-only file system
    pub const EROFS: Self = Error::new(crate::errno::EROFS);

    /// Too many links
    pub const EMLINK: Self = Error::new(crate::errno::EMLINK);

    /// Broken pipe
    pub const EPIPE: Self = Error::new(crate::errno::EPIPE);

    /// Math argument out of domain of func
    pub const EDOM: Self = Error::new(crate::errno::EDOM);

    /// Math result not representable
    pub const ERANGE: Self = Error::new(crate::errno::ERANGE);

    /// Resource deadlock would occur
    pub const EDEADLK: Self = Error::new(crate::errno::EDEADLK);

    /// File name too long
    pub const ENAMETOOLONG: Self = Error::new(crate::errno::ENAMETOOLONG);

    /// No record locks available
    pub const ENOLCK: Self = Error::new(crate::errno::ENOLCK);

    /// Invalid system call number
    pub const ENOSYS: Self = Error::new(crate::errno::ENOSYS);

    /// Directory not empty
    pub const ENOTEMPTY: Self = Error::new(crate::errno::ENOTEMPTY);

    /// Too many symbolic links encountered
    pub const ELOOP: Self = Error::new(crate::errno::ELOOP);

    /// Operation would block
    pub const EWOULDBLOCK: Self = Error::new(crate::errno::EWOULDBLOCK);

    /// No message of desired type
    pub const ENOMSG: Self = Error::new(crate::errno::ENOMSG);

    /// Identifier removed
    pub const EIDRM: Self = Error::new(crate::errno::EIDRM);

    /// Channel number out of range
    pub const ECHRNG: Self = Error::new(crate::errno::ECHRNG);

    /// Level
    pub const EL2NSYNC: Self = Error::new(crate::errno::EL2NSYNC);

    /// Level
    pub const EL3HLT: Self = Error::new(crate::errno::EL3HLT);

    /// Level
    pub const EL3RST: Self = Error::new(crate::errno::EL3RST);

    /// Link number out of range
    pub const ELNRNG: Self = Error::new(crate::errno::ELNRNG);

    /// Protocol driver not attached
    pub const EUNATCH: Self = Error::new(crate::errno::EUNATCH);

    /// No CSI structure available
    pub const ENOCSI: Self = Error::new(crate::errno::ENOCSI);

    /// Level
    pub const EL2HLT: Self = Error::new(crate::errno::EL2HLT);

    /// Invalid exchange
    pub const EBADE: Self = Error::new(crate::errno::EBADE);

    /// Invalid request descriptor
    pub const EBADR: Self = Error::new(crate::errno::EBADR);

    /// Exchange full
    pub const EXFULL: Self = Error::new(crate::errno::EXFULL);

    /// No anode
    pub const ENOANO: Self = Error::new(crate::errno::ENOANO);

    /// Invalid request code
    pub const EBADRQC: Self = Error::new(crate::errno::EBADRQC);

    /// Invalid slot
    pub const EBADSLT: Self = Error::new(crate::errno::EBADSLT);

    /// Resource deadlock would occur
    pub const EDEADLOCK: Self = Error::new(crate::errno::EDEADLOCK);

    /// Bad font file format
    pub const EBFONT: Self = Error::new(crate::errno::EBFONT);

    /// Device not a stream
    pub const ENOSTR: Self = Error::new(crate::errno::ENOSTR);

    /// No data available
    pub const ENODATA: Self = Error::new(crate::errno::ENODATA);

    /// Timer expired
    pub const ETIME: Self = Error::new(crate::errno::ETIME);

    /// Out of streams resources
    pub const ENOSR: Self = Error::new(crate::errno::ENOSR);

    /// Machine is not on the network
    pub const ENONET: Self = Error::new(crate::errno::ENONET);

    /// Package not installed
    pub const ENOPKG: Self = Error::new(crate::errno::ENOPKG);

    /// Object is remote
    pub const EREMOTE: Self = Error::new(crate::errno::EREMOTE);

    /// Link has been severed
    pub const ENOLINK: Self = Error::new(crate::errno::ENOLINK);

    /// Advertise error
    pub const EADV: Self = Error::new(crate::errno::EADV);

    /// Srmount error
    pub const ESRMNT: Self = Error::new(crate::errno::ESRMNT);

    /// Communication error on send
    pub const ECOMM: Self = Error::new(crate::errno::ECOMM);

    /// Protocol error
    pub const EPROTO: Self = Error::new(crate::errno::EPROTO);

    /// Multihop attempted
    pub const EMULTIHOP: Self = Error::new(crate::errno::EMULTIHOP);

    /// RFS specific error
    pub const EDOTDOT: Self = Error::new(crate::errno::EDOTDOT);

    /// Not a data message
    pub const EBADMSG: Self = Error::new(crate::errno::EBADMSG);

    /// Value too large for defined data type
    pub const EOVERFLOW: Self = Error::new(crate::errno::EOVERFLOW);

    /// Name not unique on network
    pub const ENOTUNIQ: Self = Error::new(crate::errno::ENOTUNIQ);

    /// File descriptor in bad state
    pub const EBADFD: Self = Error::new(crate::errno::EBADFD);

    /// Remote address changed
    pub const EREMCHG: Self = Error::new(crate::errno::EREMCHG);

    /// Can not access a needed shared library
    pub const ELIBACC: Self = Error::new(crate::errno::ELIBACC);

    /// Accessing a corrupted shared library
    pub const ELIBBAD: Self = Error::new(crate::errno::ELIBBAD);

    /// .lib section in a.out corrupted
    pub const ELIBSCN: Self = Error::new(crate::errno::ELIBSCN);

    /// Attempting to link in too many shared libraries
    pub const ELIBMAX: Self = Error::new(crate::errno::ELIBMAX);

    /// Cannot exec a shared library directly
    pub const ELIBEXEC: Self = Error::new(crate::errno::ELIBEXEC);

    /// Illegal byte sequence
    pub const EILSEQ: Self = Error::new(crate::errno::EILSEQ);

    /// Interrupted system call should be restarted
    pub const ERESTART: Self = Error::new(crate::errno::ERESTART);

    /// Streams pipe error
    pub const ESTRPIPE: Self = Error::new(crate::errno::ESTRPIPE);

    /// Too many users
    pub const EUSERS: Self = Error::new(crate::errno::EUSERS);

    /// Socket operation on non-socket
    pub const ENOTSOCK: Self = Error::new(crate::errno::ENOTSOCK);

    /// Destination address required
    pub const EDESTADDRREQ: Self = Error::new(crate::errno::EDESTADDRREQ);

    /// Message too long
    pub const EMSGSIZE: Self = Error::new(crate::errno::EMSGSIZE);

    /// Protocol wrong type for socket
    pub const EPROTOTYPE: Self = Error::new(crate::errno::EPROTOTYPE);

    /// Protocol not available
    pub const ENOPROTOOPT: Self = Error::new(crate::errno::ENOPROTOOPT);

    /// Protocol not supported
    pub const EPROTONOSUPPORT: Self = Error::new(crate::errno::EPROTONOSUPPORT);

    /// Socket type not supported
    pub const ESOCKTNOSUPPORT: Self = Error::new(crate::errno::ESOCKTNOSUPPORT);

    /// Operation not supported on transport endpoint
    pub const EOPNOTSUPP: Self = Error::new(crate::errno::EOPNOTSUPP);

    /// Protocol family not supported
    pub const EPFNOSUPPORT: Self = Error::new(crate::errno::EPFNOSUPPORT);

    /// Address family not supported by protocol
    pub const EAFNOSUPPORT: Self = Error::new(crate::errno::EAFNOSUPPORT);

    /// Address already in use
    pub const EADDRINUSE: Self = Error::new(crate::errno::EADDRINUSE);

    /// Cannot assign requested address
    pub const EADDRNOTAVAIL: Self = Error::new(crate::errno::EADDRNOTAVAIL);

    /// Network is down
    pub const ENETDOWN: Self = Error::new(crate::errno::ENETDOWN);

    /// Network is unreachable
    pub const ENETUNREACH: Self = Error::new(crate::errno::ENETUNREACH);

    /// Network dropped connection because of reset
    pub const ENETRESET: Self = Error::new(crate::errno::ENETRESET);

    /// Software caused connection abort
    pub const ECONNABORTED: Self = Error::new(crate::errno::ECONNABORTED);

    /// Connection reset by peer
    pub const ECONNRESET: Self = Error::new(crate::errno::ECONNRESET);

    /// No buffer space available
    pub const ENOBUFS: Self = Error::new(crate::errno::ENOBUFS);

    /// Transport endpoint is already connected
    pub const EISCONN: Self = Error::new(crate::errno::EISCONN);

    /// Transport endpoint is not connected
    pub const ENOTCONN: Self = Error::new(crate::errno::ENOTCONN);

    /// Cannot send after transport endpoint shutdown
    pub const ESHUTDOWN: Self = Error::new(crate::errno::ESHUTDOWN);

    /// Too many references
    pub const ETOOMANYREFS: Self = Error::new(crate::errno::ETOOMANYREFS);

    /// Connection timed out
    pub const ETIMEDOUT: Self = Error::new(crate::errno::ETIMEDOUT);

    /// Connection refused
    pub const ECONNREFUSED: Self = Error::new(crate::errno::ECONNREFUSED);

    /// Host is down
    pub const EHOSTDOWN: Self = Error::new(crate::errno::EHOSTDOWN);

    /// No route to host
    pub const EHOSTUNREACH: Self = Error::new(crate::errno::EHOSTUNREACH);

    /// Operation already in progress
    pub const EALREADY: Self = Error::new(crate::errno::EALREADY);

    /// Operation now in progress
    pub const EINPROGRESS: Self = Error::new(crate::errno::EINPROGRESS);

    /// Stale file handle
    pub const ESTALE: Self = Error::new(crate::errno::ESTALE);

    /// Structure needs cleaning
    pub const EUCLEAN: Self = Error::new(crate::errno::EUCLEAN);

    /// Not a XENIX named type file
    pub const ENOTNAM: Self = Error::new(crate::errno::ENOTNAM);

    /// No XENIX semaphores available
    pub const ENAVAIL: Self = Error::new(crate::errno::ENAVAIL);

    /// Is a named type file
    pub const EISNAM: Self = Error::new(crate::errno::EISNAM);

    /// Remote I/O error
    pub const EREMOTEIO: Self = Error::new(crate::errno::EREMOTEIO);

    /// Quota exceeded
    pub const EDQUOT: Self = Error::new(crate::errno::EDQUOT);

    /// No medium found
    pub const ENOMEDIUM: Self = Error::new(crate::errno::ENOMEDIUM);

    /// Wrong medium type
    pub const EMEDIUMTYPE: Self = Error::new(crate::errno::EMEDIUMTYPE);

    /// Operation Canceled
    pub const ECANCELED: Self = Error::new(crate::errno::ECANCELED);

    /// Required key not available
    pub const ENOKEY: Self = Error::new(crate::errno::ENOKEY);

    /// Key has expired
    pub const EKEYEXPIRED: Self = Error::new(crate::errno::EKEYEXPIRED);

    /// Key has been revoked
    pub const EKEYREVOKED: Self = Error::new(crate::errno::EKEYREVOKED);

    /// Key was rejected by service
    pub const EKEYREJECTED: Self = Error::new(crate::errno::EKEYREJECTED);

    /// Owner died
    pub const EOWNERDEAD: Self = Error::new(crate::errno::EOWNERDEAD);

    /// State not recoverable
    pub const ENOTRECOVERABLE: Self = Error::new(crate::errno::ENOTRECOVERABLE);

    /// Operation not possible due to RF-kill
    pub const ERFKILL: Self = Error::new(crate::errno::ERFKILL);

    /// Memory page has hardware error
    pub const EHWPOISON: Self = Error::new(crate::errno::EHWPOISON);
}
