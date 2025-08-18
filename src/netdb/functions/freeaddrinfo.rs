use crate::netdb::addrinfo;

extern "C" {
    /// Network address and service translation
    ///
    /// The [`freeaddrinfo`] function frees the memory that was allocated for the dynamically
    /// allocated linked list `res`.
    pub fn freeaddrinfo(res: *mut addrinfo);
}
