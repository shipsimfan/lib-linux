use crate::sys::types::uid_t;

unsafe extern "C" {
    /// Get user identity
    ///
    /// # Description
    /// [`getuid`] returns the real user ID of the calling process.
    ///
    /// # Errors
    /// This function is always successful.
    pub fn getuid() -> uid_t;
}
