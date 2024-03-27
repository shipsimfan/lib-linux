use crate::sys::socket::socklen_t;
use std::ffi::{c_int, c_void};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, EACCES, EAGAIN, EBADF, EINVAL, ENOBUFS, ENOPROTOOPT, ENOTSOCK, EWOULDBLOCK},
    sys::socket::{
        bind, SO_BROADCAST, SO_DEBUG, SO_DONTROUTE, SO_ERROR, SO_KEEPALIVE, SO_LINGER,
        SO_OOBINLINE, SO_RCVBUF, SO_RCVLOWAT, SO_RCVTIMEO, SO_REUSEADDR, SO_SNDBUF, SO_SNDLOWAT,
        SO_SNDTIMEO, SO_TYPE,
    },
    unistd::close,
};

extern "C" {
    /// Get the socket options
    ///
    /// # Description
    /// The [`getsockopt`] function manipulates options associated with a socket.
    ///
    /// The [`getsockopt`] function shall retrieve the value for the option specified by the
    /// `option_name` argument for the socket specified by the `socket` argument. If the size of
    /// the option value is greater than `option_len`, the value stored in the object pointed to
    /// by the `option_value` argument shall be silently truncated. Otherwise, the object pointed
    /// to by the `option_len` argument shall be modified to indicate the actual length of the
    /// value.
    ///
    /// The `level` argument specifies the protocol level at which the option resides. To retrieve
    /// options at the socket level, specify the level argument as [`SOL_SOCKET`]. To retrieve
    /// options at other levels, supply the appropriate level identifier for the protocol
    /// controlling the option. For example, to indicate that an option is interpreted by the TCP
    /// (Transmission Control Protocol), set level to [`IPPROTO_TCP`] as defined in
    /// [`crate::netinet::r#in`].
    ///
    /// The socket in use may require the process to have appropriate privileges to use the
    /// [`getsockopt`] function.
    ///
    /// The `option_name` argument specifies a single option to be retrieved. It can be one of the
    /// following values defined in [`crate::sys::socket`]:
    ///  * [`SO_DEBUG`] - Reports whether debugging information is being recorded. This option
    ///                   shall store a [`c_int`] value. This is a Boolean option.
    ///  * [`SO_ACCEPTCONN`] - Reports whether socket listening is enabled. This option shall store
    ///                   a [`c_int`] value. This is a Boolean option.
    ///  * [`SO_BROADCAST`] - Reports whether transmission of broadcast messages is supported, if
    ///                       this is supported by the protocol. This option shall store a
    ///                       [`c_int`] value. This is a Boolean option.
    ///  * [`SO_REUSEADDR`] - Reports whether the rules used in validating addresses supplied to
    ///                       [`bind`] should allow reuse of local addresses, if this is supported
    ///                       by the protocol. This option shall store a [`c_int`] value. This is a
    ///                       Boolean option.
    ///  * [`SO_KEEPALIVE`] - Reports whether connections are kept active with periodic
    ///                       transmission of messages, if this is supported by the protocol. If
    ///                       the connected socket fails to respond to these messages, the
    ///                       connection shall be broken and threads writing to that socket shall
    ///                       be notified with a [`SIGPIPE`] signal. This option shall store a
    ///                       [`c_int`] value. This is a Boolean option.
    ///  * [`SO_LINGER`] - Reports whether the socket lingers on [`close`] if data is present. If
    ///                    [`SO_LINGER`] is set, the system blocks the process during [`close`]
    ///                    until it can transmit the data or until the end of the interval
    ///                    indicated by the `linger` member, whichever comes first. If
    ///                    [`SO_LINGER`] is not specified, and [`close`] is issued, the system
    ///                    handles the call in a way that allows the process to continue as quickly
    ///                    as possible. This option shall store a [`linger`] structure.
    ///  * [`SO_OOBINLINE`] - Reports whether the socket leaves received out-of-band data (data
    ///                       marked urgent) inline. This option shall store a [`c_int`] value.
    ///                       This is a Boolean option.
    ///  * [`SO_SNDBUF`] - Reports send buffer size information. This option shall store a
    ///                    [`c_int`] value.
    ///  * [`SO_RCVBUF`] - Reports receive buffer size information. This option shall store a
    ///                    [`c_int`] value.
    ///  * [`SO_ERROR`] - Reports information about error status and clears it. This option shall
    ///                   store a [`c_int`] value.
    ///  * [`SO_TYPE`] - Reports the socket type. This option shall store a [`c_int`] value.
    ///  * [`SO_DONTROUTE`] - Reports whether outgoing messages bypass the standard routing
    ///                       facilities. The destination shall be on a directly-connected network,
    ///                       and messages are directed to the appropriate network interface
    ///                       according to the destination address. The effect, if any, of this
    ///                       option depends on what protocol is in use. This option shall store a
    ///                       [`c_int`] value. This is a Boolean option.
    ///  * [`SO_RCVLOWAT`] - Reports the minimum number of bytes to process for socket input
    ///                      operations. The default value for [`SO_RCVLOWAT`] is 1. If
    ///                      [`SO_RCVLOWAT`] is set to a larger value, blocking receive calls
    ///                      normally wait until they have received the smaller of the low water
    ///                      mark value or the requested amount. (They may return less than the low
    ///                      water mark if an error occurs, a signal is caught, or the type of data
    ///                      next in the receive queue is different from that returned; for
    ///                      example, out-of-band data.) This option shall store a [`c_int`] value.
    ///                      Note that not all implementations allow this option to be retrieved.
    ///  * [`SO_RCVTIMEO`] - Reports the timeout value for input operations. This option shall
    ///                      store a [`timeval`] structure with the number of seconds and
    ///                      microseconds specifying the limit on how long to wait for an input
    ///                      operation to complete. If a receive operation has blocked for this
    ///                      much time without receiving additional data, it shall return with a
    ///                      partial count or [`errno`] set to [`EAGAIN`] or [`EWOULDBLOCK`] if no
    ///                      data was received. The default for this option is zero, which
    ///                      indicates that a receive operation shall not time out. Note that not
    ///                      all implementations allow this option to be retrieved.
    ///  * [`SO_SNDLOWAT`] - Reports the minimum number of bytes to process for socket output
    ///                      operations. Non-blocking output operations shall process no data if
    ///                      flow control does not allow the smaller of the send low water mark
    ///                      value or the entire request to be processed. This option shall store
    ///                      a [`c_int`] value. Note that not all implementations allow this option
    ///                      to be retrieved.
    ///  * [`SO_SNDTIMEO`] - Reports the timeout value specifying the amount of time that an output
    ///                      function blocks because flow control prevents data from being sent. If
    ///                      a send operation has blocked for this time, it shall return with a
    ///                      partial count or with errno set to [`EAGAIN`] or [`EWOULDBLOCK`] if no
    ///                      data was sent. The default for this option is zero, which indicates
    ///                      that a send operation shall not time out. The option shall store a
    ///                      [`timeval`] structure. Note that not all implementations allow this
    ///                      option to be retrieved.
    ///
    /// For Boolean options, a zero value indicates that the option is disabled and a non-zero
    /// value indicates that the option is enabled.
    ///
    /// # Return Value
    /// Upon successful completion, [`getsockopt`] shall return 0; otherwise, -1 shall be returned
    /// and [`errno`] set to indicate the error.
    ///
    /// # Errors
    /// The [`getsockopt`] function shall fail if:
    ///  * [`EBADF`] - The `socket` argument is not a valid file descriptor.
    ///  * [`EINVAL`] - The specified option is invalid at the specified socket level.
    ///  * [`ENOPROTOOPT`] - The option is not supported by the protocol.
    ///  * [`ENOTSOCK`] - The `socket` argument does not refer to a socket.
    ///  * [`EACCES`] - The calling process does not have the appropriate privileges.
    ///  * [`EINVAL`] - The socket has been shut down.
    ///  * [`ENOBUFS`] - Insufficient resources are available in the system to complete the
    ///                  function.
    pub fn getsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *mut c_void,
        option_len: *mut socklen_t,
    ) -> c_int;
}
