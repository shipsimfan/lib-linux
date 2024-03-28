use crate::sys::socket::socklen_t;
use std::ffi::{c_int, c_void};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    errno::{errno, EAGAIN, EWOULDBLOCK},
    netinet::r#in::IPPROTO_TCP,
    sys::socket::{
        bind, SOL_SOCKET, SO_BROADCAST, SO_DEBUG, SO_DONTROUTE, SO_KEEPALIVE, SO_LINGER,
        SO_OOBINLINE, SO_RCVBUF, SO_RCVLOWAT, SO_RCVTIMEO, SO_REUSEADDR, SO_SNDBUF, SO_SNDLOWAT,
        SO_SNDTIMEO,
    },
    unistd::close,
};

extern "C" {
    /// Set the socket options
    ///
    /// # Description
    /// The [`setsockopt`] function shall set the option specified by the `option_name` argument,
    /// at the protocol level specified by the `level` argument, to the value pointed to by the
    /// `option_value` argument for the socket associated with the file descriptor specified by the
    /// `socket` argument.
    ///
    /// The `level` argument specifies the protocol level at which the option resides. To set
    /// options at the socket level, specify the level argument as [`SOL_SOCKET`]. To set options
    /// at other levels, supply the appropriate level identifier for the protocol controlling the
    /// option. For example, to indicate that an option is interpreted by the TCP (Transport
    /// Control Protocol), set level to [`IPPROTO_TCP`] as defined in [`crate::netinet::r#in`].
    ///
    /// The `option_name` argument specifies a single option to set. The `option_name` argument and
    /// any specified options are passed uninterpreted to the appropriate protocol module for
    /// interpretations. The options are as follows:
    ///  * [`SO_DEBUG`] - Turns on recording of debugging information. This option enables or
    ///                   disables debugging in the underlying protocol modules. This option takes
    ///                   a [`c_int`] value. This is a Boolean option.
    ///  * [`SO_BROADCAST`] - Permits sending of broadcast messages, if this is supported by the
    ///                       protocol. This option takes a [`c_int`] value. This is a Boolean
    ///                       option.
    ///  * [`SO_REUSEADDR`] - Specifies that the rules used in validating addresses supplied to
    ///                       [`bind`] should allow reuse of local addresses, if this is supported
    ///                       by the protocol. This option takes a [`c_int`] value. This is a
    ///                       Boolean option.
    ///  * [`SO_KEEPALIVE`] - Keeps connections active by enabling the periodic transmission of
    ///                       messages, if this is supported by the protocol. This option takes a
    ///                       [`c_int`] value. If the connected socket fails to respond to these
    ///                       messages, the connection is broken and threads writing to that socket
    ///                       are notified with a `SIGPIPE` signal. This is a Boolean option.
    ///  * [`SO_LINGER`] - Lingers on a [`close`] if data is present. This option controls the
    ///                    action taken when unsent messages queue on a socket and [`close`] is
    ///                    performed. If [`SO_LINGER`] is set, the system shall block the process
    ///                    during [`close`] until it can transmit the data or until the time
    ///                    expires. If [`SO_LINGER`] is not specified, and [`close`] is issued, the
    ///                    system handles the call in a way that allows the process to continue as
    ///                    quickly as possible. This option takes a [`linger`] structure, to
    ///                    specify the state of the option and linger interval.
    ///  * [`SO_OOBINLINE`] - Leaves received out-of-band data (data marked urgent) inline. This
    ///                       option takes a [`c_int`] value. This is a Boolean option.
    ///  * [`SO_SNDBUF`] - Sets send buffer size. This option takes a [`c_int`] value.
    ///  * [`SO_RCVBUF`] - Sets receive buffer size. This option takes a [`c_int`] value.
    ///  * [`SO_DONTROUTE`] - Requests that outgoing messages bypass the standard routing
    ///                       facilities. The destination shall be on a directly-connected network,
    ///                       and messages are directed to the appropriate network interface
    ///                       according to the destination address. The effect, if any, of this
    ///                       option depends on what protocol is in use. This option takes a
    ///                       [`c_int`] value. This is a Boolean option.
    ///  * [`SO_RCVLOWAT`] - Sets the minimum number of bytes to process for socket input
    ///                      operations. The default value for [`SO_RCVLOWAT`] is 1. If
    ///                      [`SO_RCVLOWAT`] is set to a larger value, blocking receive calls
    ///                      normally wait until they have received the smaller of the low water
    ///                      mark value or the requested amount. (They may return less than the low
    ///                      water mark if an error occurs, a signal is caught, or the type of data
    ///                      next in the receive queue is different from that returned; for
    ///                      example, out-of-band data.) This option takes a [`c_int`] value. Note
    ///                      that not all implementations allow this option to be set.
    ///  * [`SO_RCVTIMEO`] - Sets the timeout value that specifies the maximum amount of time an
    ///                      input function waits until it completes. It accepts a [`timeval`]
    ///                      structure with the number of seconds and microseconds specifying the
    ///                      limit on how long to wait for an input operation to complete. If a
    ///                      receive operation has blocked for this much time without receiving
    ///                      additional data, it shall return with a partial count or [`errno`] set
    ///                      to [`EAGAIN`] or [`EWOULDBLOCK`] if no data is received. The default
    ///                      for this option is zero, which indicates that a receive operation
    ///                      shall not time out. This option takes a [`timeval`] structure. Note
    ///                      that not all implementations allow this option to be set.
    ///  * [`SO_SNDLOWAT`] - Sets the minimum number of bytes to process for socket output
    ///                      operations. Non-blocking output operations shall process no data if
    ///                      flow control does not allow the smaller of the send low water mark
    ///                      value or the entire request to be processed. This option takes a
    ///                      [`c_int`] value. Note that not all implementations allow this option
    ///                      to be set.
    ///  * [`SO_SNDTIMEO`] - Sets the timeout value specifying the amount of time that an output
    ///                      function blocks because flow control prevents data from being sent. If
    ///                      a send operation has blocked for this time, it shall return with a
    ///                      partial count or with [`errno`] set to [`EAGAIN`] or [`EWOULDBLOCK`]
    ///                      if no data is sent. The default for this option is zero, which
    ///                      indicates that a send operation shall not time out. This option stores
    ///                      a [`timeval`] structure. Note that not all implementations allow this
    ///                      option to be set.
    ///
    /// For Boolean options, 0 indicates that the option is disabled and 1 indicates that the
    /// option is enabled.
    ///
    /// Options at other protocol levels vary in format and name.
    ///
    /// # Return Value
    /// Upon successful completion, [`setsockopt`] shall return 0. Otherwise, -1 shall be returned
    /// and [`errno`] set to indicate the error.
    ///
    /// # Errors
    /// The [`setsockopt`] function shall fail if:
    ///  * [`EBADF`] - The `socket` argument is not a valid file descriptor
    ///  * [`EDOM`] - The send and receive timeout values are too big to fit into the timeout
    ///               fields in the socket structure.
    ///  * [`EINVAL`] - The specified option is invalid at the specified socket level or the socket
    ///                 has been shut down.
    ///  * [`EISCONN`] - The socket is already connected, and a specified option cannot be set
    ///                  while the socket is connected.
    ///  * [`ENOPROTOOPT`] - The option is not supported by the protocol.
    ///  * [`ENOTSOCK`] - The `socket` argument does not refer to a socket.
    ///  * [`ENOMEM`] - There was insufficient memory available for the operation to complete.
    ///  * [`ENOBUFS`] - Insufficient resources are available in the system to complete the call.
    pub fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
}
