use crate::netdb::addrinfo;
use std::ffi::{c_char, c_int};

// rustdoc imports
#[allow(unused_imports)]
use crate::sys::socket::{
    accept, bind, connect, sendmsg, sendto, socket, AF_INET, AF_INET6, AF_UNSPEC, SOCK_DGRAM,
    SOCK_STREAM,
};
#[allow(unused_imports)]
use std::ptr::null;

extern "C" {
    /// Network address and service translation
    ///
    /// Given node and service, which identify an Internet host and a service, [`getaddrinfo`]
    /// returns one or more [`addrinfo`] structures, each of which contains an Internet address
    /// that can be specified in a call to [`bind`] or [`connect`]. The [`getaddrinfo`] function
    /// combines the functionality provided by the [`gethostbyname`] and [`getservbyname`]
    /// functions into a single interface, but unlike the latter functions, [`getaddrinfo`] is
    /// reentrant and allows programs to eliminate IPv4-versus-IPv6 dependencies.
    ///
    /// The `hints` argument points to an [`addrinfo`] structure that specifies criteria for
    /// selecting the socket address structures returned in the list pointed to by `res`. If
    /// `hints` is not [`null`] it points to an [`addrinfo`] structure whose `family`, `socktype`,
    /// `protocol`, and `flags` specify criteria that limit the set of socket addresses returned by
    /// [`getaddrinfo`].
    ///
    /// All the other fields in the structure pointed to by `hints` must contain either 0 or a
    /// [`null`] pointer, as appropriate.
    ///
    /// Specifying `hints` as [`null`] is equivalent to setting `socktype` and `protocol` to 0;
    /// `family` to [`AF_UNSPEC`]; and `flags` to ([`AI_V4MAPPED`] | [`AI_ADDRCONFIG`]). (POSIX
    /// specifies different defaults for `flags`) `node` specifies either a numerical network
    /// address (for IPv4, numbers-and-dots notation as supported by [`inet_aton`]); for IPv6,
    /// hexadecimal string format as supported by [`inet_pton`]), or a network hostname, whose
    /// network addresses are looked up and resolved. If `hints.flags` contains the
    /// [`AI_NUMERICHOST`] flag, then node must be a numerical network address. The
    /// [`AI_NUMERICHOST`] flag suppresses any potentially lengthy network host address lookups.
    ///
    /// If the [`AI_PASSIVE`] flag is specified in `hints.flags`, and `node` is [`null`], then the
    /// returned socket addresses will be suitable for [`bind`]ing a socket that will [`accept`]
    /// connections. The returned socket address will contain the "wildcard address"
    /// ([`INADDR_ANY`] for IPv4 addresses, [`IN6ADDR_ANY_INIT`] for IPv6 address). The wildcard
    /// address is used by applications (typically servers) that intend to accept connections on
    /// any of the host's network addresses. If `node` is not [`null`], then the [`AI_PASSIVE`]
    /// flag is ignored.
    ///
    /// If the [`AI_PASSIVE`] flag is not set in `hints.flags`, then the returned socket addresses
    /// will be suitable for use with [`connect`], [`sendto`], or [`sendmsg`]. If `node` is
    /// [`null`], then the network address will be set to the loopback interface address
    /// ([`INADDR_LOOPBACK`] for IPv4 addresses, [`IN6ADDR_LOOPBACK_INIT`] for IPv6 address); this
    /// is used by applications that intend to communicate with peers running on the same host.
    ///
    /// `service` sets the port in each returned address structure. If this argument is a service
    /// name, it is translated to the corresponding port number. This argument can also be
    /// specified as a decimal number, which is simply converted to binary. If `service` is
    /// [`null`], then the port number of the returned socket addresses will be left uninitialized.
    /// If [`AI_NUMERICSERV`] is specified in `hints.flags` and `service` is not [`null`], then
    /// `service` must point to a string containing a numeric port number. This flag is used to
    /// inhibit the invocation of a name resolution service in cases where it is known not to be
    /// required.
    ///
    /// Either `node` or `service`, but not both, may be [`null`].
    ///
    /// The [`getaddrinfo`] function allocates and initializes a linked list of [`addrinfo`]
    /// structures, one for each network address that matches `node` and `service`, subject to any
    /// restrictions imposed by `hints`, and returns a pointer to the start of the list in `res`.
    /// The items in the linked list are linked by the `next` field.
    ///
    /// There are several reasons why the linked list may have more than one [`addrinfo`]
    /// structure, including: the network host is multihomed, accessible over multiple protocols
    /// (e.g., both [`AF_INET`] and [`AF_INET6`]); or the same service is available from multiple
    /// socket types (one [`SOCK_STREAM`] address and another [`SOCK_DGRAM`] address, for example).
    /// Normally, the application should try using the addresses in the order in which they are
    /// returned. The sorting function used within [`getaddrinfo`] is defined in RFC 3484; the
    /// order can be tweaked for a particular system by editing "/etc/gai.conf" (available since
    /// glibc 2.5).
    ///
    /// If `hints.flags` includes the [`AI_CANONNAME`] flag, then the `canonname` field of the
    /// first of the [`addrinfo`] structures in the returned list is set to point to the official
    /// name of the host.
    ///
    /// The remaining fields of each returned [`addrinfo`] structure are initialized as follows:
    ///  - The `family`, `socktype`, and `protocol` fields return the socket creation parameters
    ///    (i.e., these fields have the same meaning as the corresponding arguments of [`socket`]).
    ///    For example, `family` might return [`AF_INET`] or [`AF_INET6`]; `socktype` might return
    ///    [`SOCK_DGRAM`] or [`SOCK_STREAM`]; and `protocol` returns the protocol for the socket.
    ///  - A pointer to the socket address is placed in the `addr` field, and the size of the
    ///    socket address, in bytes, is placed in the `addrlen` field.
    ///
    /// If `hints.flags` includes the [`AI_ADDRCONFIG`] flag, then IPv4 addresses are returned in
    /// the list pointed to by `res` only if the local system has at least one IPv4 address
    /// configured, and IPv6 addresses are returned only if the local system has at least one IPv6
    /// address configured. The loopback address is not considered for this case as valid as a
    /// configured address. This flag is useful on, for example, IPv4-only systems, to ensure that
    /// [`getaddrinfo`] does not return IPv6 socket addresses that would always fail in [`connect`]
    /// or [`bind`].
    ///
    /// If `hints.flags` specifies the [`AI_V4MAPPED`] flag, and `hints.family` was specified as
    /// [`AF_INET6`], and no matching IPv6 addresses could be found, then return IPv4-mapped IPv6
    /// addresses in the list pointed to by `res`. If both [`AI_V4MAPPED`] and [`AI_ALL`] are
    /// specified in `hints.flags`, then return both IPv6 and IPv4-mapped IPv6 addresses in the
    /// list pointed to by `res`. [`AI_ALL`] is ignored if [`AI_V4MAPPED`] is not also specified.
    ///
    /// # Return Value
    /// [`getaddrinfo`] returns 0 if it succeeds, or a nonzero error code.
    pub fn getaddrinfo(
        node: *const c_char,
        service: *const c_char,
        hints: *const addrinfo,
        res: *mut *mut addrinfo,
    ) -> c_int;
}
