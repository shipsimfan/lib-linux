use crate::netdb::addrinfo;

unsafe extern "C" {
    /// Network address and service translation
    ///
    /// # Description
    /// The [`freeaddrinfo`] function frees the memory that was allocated for the dynamically
    /// allocated linked list `res`.
    pub fn freeaddrinfo(res: *mut addrinfo);
}
