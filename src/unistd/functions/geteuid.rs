use crate::sys::types::uid_t;

unsafe extern "C" {
    /// Get user identity
    ///
    /// # Description
    /// [`geteuid`] returns the effective user ID of the calling process.
    ///
    /// # Errors
    /// This function is always successful.
    pub fn geteuid() -> uid_t;
}
